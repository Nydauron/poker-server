// TODO: Figure out how to implement authorization checks thru JWT tokens (i.e. check a Bearer token from the Authorization header)
use actix_web::{web, get, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use crate::websocket::{PlayerSocket, PlayerSocketManager};
use crate::poker::Table;
use crate::poker::Payload;

use tokio::sync::{mpsc, watch};

mod websocket;

mod poker;

#[derive(Clone)]
struct ChannelData {
    tx: mpsc::UnboundedSender<Payload>,
    rx: watch::Receiver<Payload>
}

#[get("/ws/")]
async fn handle_websocket(data: web::Data<ChannelData>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(PlayerSocket::new(data.tx.clone(), data.rx.clone()), &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(init()))
        .service(handle_websocket))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

fn init() -> ChannelData {
    let (req_tx, mut req_rx) = mpsc::unbounded_channel::<Payload>();
    let (res_tx, res_rx) = watch::channel::<Payload>(0);

    {
        let (gl_tx, mut gl_rx) = mpsc::unbounded_channel::<Payload>();

        // Websocket listener (listens to the mpsc channels, parses them, and funnels them into a oneshot channel for the game loop)
        tokio::task::spawn(async move {
            PlayerSocketManager::listener(gl_tx, &mut req_rx).await;
        });

        // Game loop
        tokio::task::spawn_blocking(move || { // gotta benchmark to see if block_in_place provides any speedup
            let mut table = Table::new();
            table.run_loop(&mut gl_rx, res_tx);
        });
    }

    ChannelData {
        tx: req_tx.clone(),
        rx: res_rx.clone(),
    }
}
