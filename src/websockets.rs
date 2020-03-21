
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web::{web, HttpResponse, HttpRequest, Error};
use actix_web_actors::ws;

use log::{info, debug};
use super::data::Datasources;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket<'a> {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    data: web::Data<Datasources<'a>>,
}

impl Actor for MyWebSocket<'static> {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket<'static> {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
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
        }
    }
}

impl MyWebSocket<'static> {
    fn new(x: web::Data<Datasources<'static>>) -> Self {
        Self {
            hb: Instant::now(),
            data: x
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) -> () {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Websocket Client hearbeat failed: disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a pint
                return;
            }
            ctx.ping(b"");
        });
    }
}

pub async fn ws_index(ds: web::Data<Datasources<'static>>, r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let res = ws::start(MyWebSocket::new(ds), &r, stream);
    res
}
