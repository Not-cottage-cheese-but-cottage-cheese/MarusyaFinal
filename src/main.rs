use actix::{Actor, Addr, Context, Handler};
use actix_cors::Cors;
use actix_web::{middleware, post, web, App, HttpServer, Responder};

pub mod session;
pub mod webhook;

use self::session::{
    games::*,
    messages::*,
    SessionEvents,
};
use webhook::Request;

async fn game<T>(
    sessions: web::Data<Addr<SessionEvents>>,
    request: web::Json<Request>,
    actor: T,
) -> impl Responder
where
    T: Actor<Context = Context<T>> + Handler<GameMessage>,
{
    let response_builder = webhook::response::ResponseBuilder::new();

    let session_id = request.session.session_id.clone();

    let is_new_sess = is_new_session(&sessions, session_id.clone(), &request).await;
    let is_start_sess = is_start_session(&request).await;

    let response = if is_new_sess && is_start_sess {
        let res = sessions
            .send(Subscribe {
                session_id: session_id,
                recipient: actor.start().recipient(),
            })
            .await;

        match res {
            Ok(res) => res,
            Err(e) => Err(format!("{}", e)),
        }
    } else if !is_new_sess {
        let res = sessions
            .send(GameMessage::Move(GameMove {
                session_id: session_id,
                data: request.request.clone(),
            }))
            .await;

        match res {
            Ok(res) => res,
            Err(e) => Err(format!("{}", e)),
        }
    } else {
        Err("Неизвестная команда".into())
    };

    web::Json(
        response_builder
            .set_response(response)
            .set_session(
                request.session.session_id.clone(),
                request.session.user_id.clone(),
                request.session.message_id,
            )
            .set_version(request.version.clone())
            .build()
            .unwrap(),
    )
}

#[post("/21")]
async fn game_21(
    sessions: web::Data<Addr<SessionEvents>>,
    request: web::Json<Request>,
) -> impl Responder {
    game(sessions, request, Game21::new()).await
}

#[post("/edible")]
async fn game_edible(
    sessions: web::Data<Addr<SessionEvents>>,
    request: web::Json<Request>,
) -> impl Responder {
    game(sessions, request, GameEdible::new()).await
}

async fn is_new_session(
    sessions: &web::Data<Addr<SessionEvents>>,
    session_id: String,
    _request: &Request,
) -> bool {
    !sessions.send(IsSubscribed(session_id)).await.unwrap()
}

async fn is_start_session(request: &Request) -> bool {
    request.get_nlu().contains(&"старт".into())
}

#[actix_web::main]
async fn main() {
    std::env::set_var(
        "RUST_LOG",
        "debug,my_errors=debug,actix_server=debug,actix_web=debug",
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

	let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let sessions = SessionEvents::new().start();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .data(sessions.clone())
            .service(game_21)
			.service(game_edible)
    })
    .bind(("0.0.0.0", port))
	.expect(format!("Can not bind to port {}", port))
    .run()
	.await;
}
