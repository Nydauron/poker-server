// TODO: Figure out how to implement authorization checks thru JWT tokens (i.e. check a Bearer token from the Authorization header)
use actix_web::{web, get, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use actix::{Actor, Addr};
use crate::websocket::PlayerSocket;
use crate::poker::Table;
use crate::poker::GameActionPayload;
use crate::poker::MessageManager;
use crate::poker::GameActionResponse;

use tokio::sync::mpsc;

use std::sync::{Arc, Mutex};

mod websocket;

mod poker;

// #[derive(Clone)]
// struct ChannelData {
//     tx_addr: Addr<MessageManager>,
// }

type ChannelData = Addr<MessageManager>;

#[get("/ws/")]
async fn handle_websocket(data: web::Data<ChannelData>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(PlayerSocket::new(data.get_ref()), &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = init();

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(data.clone()))
        .service(handle_websocket))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

fn init() -> ChannelData {
    let (gl_tx, mut gl_rx) = mpsc::unbounded_channel::<GameActionPayload>();
    let (res_tx, mut res_rx) = mpsc::unbounded_channel::<GameActionResponse>();

    let msg_man = MessageManager::new(gl_tx);
    let channel_data = msg_man.start();

    let msg_addr = channel_data.clone();
    
    // Websocket listener (listens to the mpsc channels, parses them, and funnels them into a oneshot channel for the game loop)
    tokio::task::spawn(async move {
        MessageManager::listener(msg_addr, &mut res_rx).await;
    });

    // Game loop
    tokio::task::spawn(async move { // gotta benchmark to see if block_in_place provides any speedup
        // in reality, it makes sense for this thread to be async since we are waiting for I/O
        let table = Arc::new(Mutex::new(Table::new()));
        Table::run_loop(table, &mut gl_rx, res_tx).await;
    });

    channel_data
}
