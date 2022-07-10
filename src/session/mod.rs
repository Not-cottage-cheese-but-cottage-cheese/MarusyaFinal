pub mod games;
pub mod messages;

use actix::prelude::{Actor, Context, Handler, Recipient};
use futures::executor::block_on;
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

    fn handle(&mut self, msg: GameMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let message = msg.clone();
        let sess_id = match msg {
            GameMessage::Stop(session_id) => session_id,
            GameMessage::Move(ref gm) => gm.session_id.clone(),
            _ => unreachable!(),
        };

        match self.subscribers.entry(sess_id) {
            Entry::Occupied(entry) => match block_on(entry.get().send(message)) {
                Ok(mess_res) => {
                    let should_delete = mess_res
                        .clone()
                        .ok()
                        .and_then(|res_obj| {
                            res_obj
                                .as_object()
                                .and_then(|res_obj| res_obj.get("end_session"))
                                .and_then(|is_end| is_end.as_bool())
                                .filter(|is_end| *is_end)
                        })
                        .is_some();

                    if should_delete {
                        entry.remove_entry();
                    }
                    mess_res
                }
                Err(e) => Err(format!("{}", e)),
            },
            Entry::Vacant(_) => Err("Сессии с выбранным id не существует".into()),
        }
        // self.subscribers
        //     .get(&sess_id)
        //     .map(|rec| match block_on(rec.send(message)) {
        //         Ok(mess_res) => mess_res,
        //         Err(e) => Err(format!("{}", e)),
        //     })
        //     .unwrap_or(Err("Сессии с выбранным id не существует".into()))
    }
}

impl Handler<Subscribe> for SessionEvents {
    type Result = Result<JsonValue, String>;

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) -> Self::Result {
        log::debug!("New session started");

        match self.subscribers.entry(msg.session_id.clone()) {
            Entry::Occupied(mut entry) => {
                let res = entry
                    .get()
                    .do_send(GameMessage::Stop(entry.key().clone()))
                    .map_err(|e| format!("{}", e));
                if res.is_ok() {
                    entry.insert(msg.recipient.clone());
                }

                match block_on(msg.recipient.send(GameMessage::Started)) {
                    Ok(res) => res,
                    Err(e) => Err(format!("{}", e)),
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(msg.recipient.clone());

                match block_on(msg.recipient.send(GameMessage::Started)) {
                    Ok(res) => res,
                    Err(e) => Err(format!("{}", e)),
                }
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
