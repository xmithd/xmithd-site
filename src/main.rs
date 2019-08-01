mod constants;
mod routes;
mod data;

use std::{io};

extern crate log;
extern crate env_logger;
extern crate handlebars;

use log::{info};

use actix_files as fs;

#[macro_use]
extern crate actix_web;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

#[macro_use]
extern crate serde_json;
extern crate serde_derive;

fn main() -> io::Result<()> {
  env_logger::init();
  let state = data::Datasources::new();
  let addr = format!("{}:{}", state.conf().host, state.conf().port);
  info!("Starting http server at http://{}", &addr);

  let datasources_ref = web::Data::new(state);

  // TODO add error handling middleware

  HttpServer::new( move || {
    App::new()
        .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
        .register_data(datasources_ref.clone())
        .service(routes::home)
        .service(routes::apps)
        .service(routes::about)
        .service(routes::contact)
        .service(fs::Files::new("/", constants::PUBLIC_FOLDER))
  })
  .bind(&addr)?
  .run()
}
