use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::{middleware, post, web, App, HttpServer, Responder};

pub mod session;
pub mod webhook;

use self::session::{
    games::{Game21, GameMove},
    messages::*,
    SessionEvents,
};
use webhook::Request;

#[post("/21")]
async fn game_21(
    sessions: web::Data<Addr<SessionEvents>>,
    request: web::Json<Request>,
) -> impl Responder {
    let response_builder = webhook::response::ResponseBuilder::new();

    let session_id = request.session.session_id.clone();
    if is_new_session(&sessions, session_id.clone(), &request).await {
        sessions
            .send(Subscribe {
                session_id: session_id,
                recipient: Game21::new().start().recipient(),
            })
            .await;
    } else {
        sessions
            .send(GameMessage::Move(GameMove {
                session_id: session_id,
                data: request.request.clone(),
            }))
            .await;
    }

    web::Json(
        response_builder
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

async fn is_new_session(
    sessions: &web::Data<Addr<SessionEvents>>,
    session_id: String,
    request: &Request,
) -> bool {
    !sessions.send(IsSubscribed(session_id)).await.unwrap()
        && request.get_nlu().contains(&"старт".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "debug,my_errors=debug,actix_server=debug,actix_web=debug",
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

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
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
