// TODO: Figure out how to implement authorization checks thru JWT tokens (i.e. check a Bearer token from the Authorization header)
use actix_web::{web, get, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use crate::websocket::PlayerSocket;
use crate::poker::CardDeck;

mod websocket;

mod poker;

#[get("/ws/")]
async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(PlayerSocket {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}