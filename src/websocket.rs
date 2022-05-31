use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::poker::{MessageManager, WebsocketConnect, WebsocketDisconnect, SendSingleResponse, ActionResponse};

#[derive(Message)]
#[rtype(result = "()")]
struct Listener;

pub struct PlayerSocket {
    id: Uuid,
    tx_addr: Addr<MessageManager>,
}

impl PlayerSocket {
    pub fn new(tx_addr: &Addr<MessageManager>) -> PlayerSocket {
        PlayerSocket {
            id: Uuid::new_v4(),
            tx_addr: tx_addr.clone(),
        }
    }
}

impl Actor for PlayerSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // TODO: Add a heartbeat function

        let addr = ctx.address();
        self.tx_addr
            .send(
                WebsocketConnect{
                    id: self.id,
                    ws_addr: addr.recipient(),
                }
            )
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop()
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.tx_addr.do_send( WebsocketDisconnect { ws_uuid: self.id } );
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlayerSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => { println!("received ping"); ctx.pong(&msg); },
            Ok(ws::Message::Text(text)) => {
                println!("received text");
                self.tx_addr.do_send( SendSingleResponse{sender_uuid: self.id, response: ActionResponse("hello".to_owned())} )
            },
            Ok(ws::Message::Binary(bin)) => { println!("received bytes"); ctx.binary(bin); },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Response(pub String);

impl Handler<Response> for PlayerSocket {
    type Result = ();

    fn handle(&mut self, msg: Response, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}
