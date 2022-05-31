use std::collections::HashMap;
use uuid::Uuid;

use actix::prelude::{Actor, Context, Handler, Message};
use actix::{Addr, Recipient};

use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};

use crate::websocket::{Response};

pub type Payload = i64;
// This module is in charge of turning incoming messages into Action structs
// of which will be forwarded to the table game loop

pub type GameState = i64;

pub struct ActionResponse(pub String);

#[derive(Debug)]
pub struct MessageManager{
    pub tx: UnboundedSender<Payload>, // tx to send info to game
    // pub rx: UnboundedReceiver<Payload>, // rx to receive info from game (channel is instead sent to static listener method)
    sockets: HashMap<Uuid, Recipient<Response>>
}

impl MessageManager {
    pub fn new(tx: UnboundedSender<Payload>) -> Self {
        let me = Self {
            tx: tx,
            sockets: HashMap::new(),
        };
        // println!("{:#?}", me);
        me
    }

    pub fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(s) = self.sockets.get(id_to) {
            let _ = s.do_send(Response(message.to_owned()));
        }
    }

    pub async fn listener(addr: Addr<MessageManager>, rx: &mut UnboundedReceiver<Payload>) {
        while let Some(msg) = rx.recv().await {
            // addr.send(BroadcastState{});
        }
    }
}

impl Actor for MessageManager {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WebsocketConnect {
    pub id: Uuid,
    pub ws_addr: Recipient<Response>,
}

impl Handler<WebsocketConnect> for MessageManager {
    type Result = ();

    fn handle(&mut self, msg: WebsocketConnect, _: &mut Self::Context) {
        self.sockets.insert(msg.id, msg.ws_addr);

        self.send_message("Hello! and welcome to the poker server!", &msg.id);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WebsocketDisconnect {
    pub ws_uuid: Uuid,
}

impl Handler<WebsocketDisconnect> for MessageManager {
    type Result = ();

    fn handle(&mut self, msg: WebsocketDisconnect, _: &mut Self::Context) {
        if self.sockets.remove(&msg.ws_uuid).is_some() {
            // do other stuff here ig like mark the player as offline (send that to the table)
            // could also broadcast to other nodes that node x has disconnected
            self.sockets.iter().for_each(|id|
                self.send_message("A node disconnected", id.0)
            )
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastState {
    pub sender_uuid: Uuid,
    pub state: GameState,
}

impl Handler<BroadcastState> for MessageManager {
    type Result = ();

    fn handle(&mut self, msg: BroadcastState, _: &mut Self::Context) {

    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendSingleResponse {
    pub sender_uuid: Uuid,
    pub response: ActionResponse, // create a new type for this
}

impl Handler<SendSingleResponse> for MessageManager {
    type Result = ();

    fn handle(&mut self, msg: SendSingleResponse, _: &mut Self::Context) {
        // self.sockets.iter().for_each(|id|
        //     self.send_message(&msg.response.0, id.0)
        // )
    }
}
