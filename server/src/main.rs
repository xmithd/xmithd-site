mod config;

mod routes;

use std::{io};

extern crate log;

extern crate env_logger;

use log::{info};

use actix_files as fs;

#[macro_use]
extern crate actix_web;

use actix_web::{web, App, HttpServer, Result};
use actix_web::middleware::Logger;

use handlebars::Handlebars;

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
  let addr = format!("{}:{}", config::HOST, config::PORT);
  info!("Starting http server at http://{}", &addr);
  // Handlebars uses a repository for the compiled templates. This object must be
  // shared between the application threads, and is therefore passed to the
  // Application Builder as an atomic reference-counted pointer.
  let mut handlebars = Handlebars::new();
  handlebars
      .register_templates_directory(".hbs", "./static/templates")
      .unwrap();
  handlebars.set_strict_mode(true);
  let handlebars_ref = web::Data::new(handlebars);

  HttpServer::new( move || {
    App::new()
        .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
        .register_data(handlebars_ref.clone())
        .service(routes::home)
        .service(routes::apps)
        .service(web::resource("/chatapp").route(web::get().to(|| ui_app(config::REALTIME_CHAT_APP_ROOT))))
        .service(fs::Files::new("/chatapp", config::REALTIME_CHAT_APP_ROOT))
        .service(fs::Files::new("/", config::PUBLIC_FOLDER))
  })
  .bind(&addr)?
  .run()
}

