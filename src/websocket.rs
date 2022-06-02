use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use serde::{Serialize, Deserializer};
use uuid::Uuid;

use crate::poker::{MessageManager, WebsocketConnect, WebsocketDisconnect, SendSingleResponse, ActionRequest};

use serde_json::{Value, value, json, error::Error};

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
                println!("Recieved: {}", text.to_string());
                let res = serde_json::from_str(&text.to_string());
                match res {
                    Ok(Value::Object(mut res)) => {
                        res.insert("id".to_string(), json!(self.id));

                        let action: Result<ActionRequest, Error> = value::from_value(Value::Object(res));
                        match action {
                            Ok(action) => {
                                println!("{:#?}", action);
                                self.tx_addr.do_send(action);
                            },
                            Err(err) => {
                                let res = WebsocketResponse{
                                    action_type: "Action parse error".to_string(),
                                    error: Some(err.to_string()),
                                    data: json!({}),
                                };
                                ctx.address().do_send(res);
                            }
                        }
                    },
                    Ok(_) => {
                        let res = WebsocketResponse{
                            action_type: "JSON parse error".to_string(),
                            error: Some("Root of JSON was not of object type".to_string()),
                            data: json!({}),
                        };
                        ctx.address().do_send(res);
                    },
                    Err(err) => {
                        let res = WebsocketResponse{
                            action_type: "JSON parse error".to_string(),
                            error: Some(err.to_string()),
                            data: json!({}),
                        };
                        ctx.address().do_send(res);
                    },
                }
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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct WebsocketResponse {
    pub action_type: String,
    pub error: Option<String>,
    pub data: Value,
}

impl Handler<WebsocketResponse> for PlayerSocket {
    type Result = ();

    fn handle(&mut self, msg: WebsocketResponse, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}
