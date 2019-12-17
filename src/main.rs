mod constants;
mod routes;
mod data;
mod entity;

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

//#[macro_use]
//extern crate serde_json;
//extern crate serde_derive;

/*
fn render_index(folder_path: &str) -> Result<fs::NamedFile> {
  let file = fs::NamedFile::open(format!("{}/index.html", folder_path))?;
  Ok(file)
}
*/

fn main() -> io::Result<()> {
  env_logger::init();
  let state = data::Datasources::new();
  let addr = format!("{}:{}", state.conf().host, state.conf().port);
  info!("Starting http server at http://{}", &addr);
  let static_files_path = String::from(&state.conf().static_files);
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
        .service(routes::blog)
        //.service(routes::close_db)
        .service(routes::user_list)
        .service(routes::post_raw)
        //.service(web::resource("/").route(web::get().to(|| render_index(constants::PUBLIC_FOLDER))))
        .service(fs::Files::new("/public/", &static_files_path))
        .service(fs::Files::new("/", constants::PUBLIC_FOLDER))
  })
  .bind(&addr)?
  .run()
}

