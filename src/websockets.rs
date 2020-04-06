
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web::{web, HttpResponse, HttpRequest, Error};
use actix_web_actors::ws;

use log::{info, debug, error, trace};
use super::data::Datasources;

use super::data::chat_broker;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it's easier
/// to handle with an actor
struct MyWebSocket<'a> {
    /// unique session id
    id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    /// Joined room
    room: String,
    /// peer name
    name: Option<String>,
    /// Data source (note: broker is under data.broker())
    data: web::Data<Datasources<'a>>,
}

impl Actor for MyWebSocket<'static> {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        // start the heartbeat process on session start
        self.hb(ctx);

        // register self in chat broker. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        let addr = ctx.address();
        self.data.broker().send(
            chat_broker::Connect {
                addr: addr.recipient(),
            }
        ).into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            }).wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat broker
        self.data.broker().do_send(chat_broker::Disconnect { id: self.id });
        Running::Stop
    }
}

// Handle messages from chat broker, forward the message to peer websocker
impl Handler<chat_broker::Message> for MyWebSocket<'static> {
    type Result = ();

    fn handle(&mut self, msg: chat_broker::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket<'static> {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // Echo below is commented
        /*match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                ctx.text(text)
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => ctx.stop(),
        }*/
        let msg = match msg {
            Err(e) => {
                debug!("Error {:?} occurred", e);
                ctx.stop();
                return;
            },
            Ok(msg) => msg,
        };
        trace!("WS message: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            },
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            },
            ws::Message::Text(text) => {
                let m = text.trim();
                // check for commands starting with '/'
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    match v[0] {
                        "/list" => {
                            // Send ListRooms message the chat broker and wait for response
                            debug!("List rooms");
                            // it would be nice if we could await here:
                            self.data.broker().send(chat_broker::ListRooms)
                                .into_actor(self)
                                .then(|res, _, ctx| {
                                    match res {
                                        Ok(rooms) => {
                                            for room in rooms {
                                                ctx.text(room);
                                            }
                                        },
                                        _ => {
                                            error!("Something went wrong while getting room list...");
                                        }
                                    }
                                    fut::ready(())
                                })
                                .wait(ctx)
                            // .wait(ctx) pauses all events in context,
                            // so actor wont receive any new messages until it get list
                            // of rooms back
                        },
                        "/join" => {
                            if v.len() == 2 {
                                self.room = v[1].to_owned();
                                self.data.broker().do_send(chat_broker::Join {
                                    id: self.id,
                                    name: self.room.clone(),
                                });
                                ctx.text(format!("user {} joined", self.id));
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        },
                        "/name" => {
                            if v.len() == 2 {
                                self.name = Some(v[1].to_owned());
                            } else {
                                ctx.text("!!! name is required");
                            }
                        },
                        _ => {
                            debug!("Invalid command {}", m);
                            ctx.text(format!("!!! unknown command: {:?}", m))
                        },
                    }
                } else {
                    // just a regular message
                    let msg = if let Some(ref name) = self.name {
                        format!("{}: {}", name, m)
                    } else {
                        m.to_owned()
                    };
                    // send message to chat broker
                    self.data.broker().do_send(chat_broker::ClientMessage {
                        id: self.id,
                        msg,
                        room: self.room.clone(),
                    })
                }
            },
            ws::Message::Binary(_) => {
                info!("Unexpected binary from user {}", &self.id);
            },
            ws::Message::Close(_) => {
                ctx.stop();
            },
            ws::Message::Continuation(_) => {
                ctx.stop();
            },
            ws::Message::Nop => (),
        }
    }
}

impl MyWebSocket<'static> {
    fn new(x: web::Data<Datasources<'static>>) -> Self {
        Self {
            id: 0,
            room: "Main".to_owned(),
            name: None,
            hb: Instant::now(),
            data: x
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) -> () {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Websocket Client heartbeat failed: disconnecting!");

                // stop actor
                ctx.stop();
            } else {
                ctx.ping(b"");
            }
        });
    }
}

pub async fn ws_index(ds: web::Data<Datasources<'static>>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let my_socket = MyWebSocket::new(ds);
    let res = ws::start(my_socket, &req, stream);
    res
}
