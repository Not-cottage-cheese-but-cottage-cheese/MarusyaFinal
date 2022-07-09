pub mod games;
pub mod messages;

use actix::prelude::{Actor, ActorContext, Context, Handler, Recipient};
use serde_json::Value as JsonValue;
use std::collections::{hash_map::Entry, HashMap};

use self::messages::*;

#[derive(Default)]
pub struct SessionEvents {
    subscribers: HashMap<String, Recipient<GameMessage>>,
}

impl SessionEvents {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Handler<GameMessage> for SessionEvents {
    type Result = Result<JsonValue, String>;

    fn handle(&mut self, msg: GameMessage, ctx: &mut Context<Self>) -> Self::Result {
        let message = msg.clone();
        let sess_id = match msg {
            GameMessage::Stop(session_id) => session_id,
            GameMessage::Move(ref gm) => gm.session_id.clone(),
        };

        use futures::executor::block_on;
        self.subscribers
            .get(&sess_id)
            .map(|rec| match block_on(rec.send(message)) {
                Ok(mess_res) => mess_res,
                Err(e) => Err(format!("{}", e)),
            })
            .unwrap_or(Err("Сессии с выбранным id не существует".into()))
    }
}

impl Handler<Subscribe> for SessionEvents {
    type Result = Result<JsonValue, String>;

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) -> Self::Result {
        match self.subscribers.entry(msg.session_id.clone()) {
            Entry::Occupied(mut entry) => {
                let res = entry
                    .get()
                    .do_send(GameMessage::Stop(entry.key().clone()))
                    .map_err(|e| format!("{}", e));
                if res.is_ok() {
                    entry.insert(msg.recipient);
                }

				// TODO - send correct result
				unimplemented!()
            }
            Entry::Vacant(entry) => {
                entry.insert(msg.recipient);
				
				unimplemented!();
            }
        }
    }
}

impl Handler<IsSubscribed> for SessionEvents {
    type Result = bool;

    fn handle(&mut self, msg: IsSubscribed, _: &mut Self::Context) -> Self::Result {
        self.subscribers.contains_key(&msg.0)
    }
}

impl Actor for SessionEvents {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::debug!("Session Events Started");
    }
}
