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

use actix_web::{web, App, HttpServer, Result};
use actix_web::middleware::Logger;

#[macro_use]
extern crate serde_json;

fn render_file(file_name: &str) -> Result<fs::NamedFile> {
  let file = fs::NamedFile::open(file_name)?;
  Ok(file)
}

fn ui_app(webapp_root: &str) -> Result<fs::NamedFile> {
  let index_path= format!("{}/index.html", webapp_root);
  render_file(&index_path)
}

fn main() -> io::Result<()> {
  env_logger::init();
  let addr = format!("{}:{}", constants::HOST, constants::PORT);
  info!("Starting http server at http://{}", &addr);

  let datasources_ref = web::Data::new(data::Datasources::new());

  // TODO add error handling middleware

  HttpServer::new( move || {
    App::new()
        .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
        .register_data(datasources_ref.clone())
        .service(routes::home)
        .service(routes::apps)
        .service(routes::about)
        .service(routes::contact)
        .service(web::resource("/chatapp").route(web::get().to(|| ui_app(constants::REALTIME_CHAT_APP_ROOT))))
        .service(fs::Files::new("/chatapp", constants::REALTIME_CHAT_APP_ROOT))
        .service(fs::Files::new("/", constants::PUBLIC_FOLDER))
  })
  .bind(&addr)?
  .run()
}

