use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use tokio::sync::watch::Receiver;

use crate::poker::Payload;

pub struct PlayerSocket {
    tx: UnboundedSender<Payload>,
    rx: Receiver<Payload>
}

impl PlayerSocket {
    pub fn new(tx: UnboundedSender<Payload>, rx: Receiver<Payload>) -> PlayerSocket {
        PlayerSocket {
            tx: tx,
            rx: rx,
        }
    }
}

impl Actor for PlayerSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlayerSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub struct PlayerSocketManager;

impl PlayerSocketManager {
    pub async fn listener(tx: UnboundedSender<Payload>, mut rx: &mut UnboundedReceiver<Payload>) {
        while let Some(msg) = rx.recv().await {
            let err = tx.send(1337).unwrap_err();
        }
    }
}
