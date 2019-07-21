extern crate actix_web;

mod config;

use std::{io};

use actix_files as fs;
use actix_web::{
  web, App, HttpServer,
  Result,
};
use actix_web::middleware::Logger;

fn render_file(file_name: &str) -> Result<fs::NamedFile> {
  let file = fs::NamedFile::open(file_name)?;
  Ok(file)
}

fn ui_app(webapp_root: &str) -> Result<fs::NamedFile> {
  let index_path= format!("{}/index.html", webapp_root);
  render_file(&index_path)
}

fn home_page() -> Result<fs::NamedFile> {
  render_file("./templates/home.html")
}

fn favicon() -> Result<fs::NamedFile> {
  render_file("./templates/favicon.ico")
}

fn main() -> io::Result<()> {
  let addr = format!("127.0.0.1:{}", config::PORT);
  println!("Starting http server at http://{}", &addr);


  HttpServer::new( || {
    App::new()
        .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
    .service(web::resource("/").route(web::get().to(home_page)))
    .service(web::resource("/favicon.ico").route(web::get().to(favicon)))
    .service(web::resource("/chatapp").route(web::get().to(|| ui_app(config::REALTIME_CHAT_APP_ROOT))))
    .service(fs::Files::new("/chatapp", config::REALTIME_CHAT_APP_ROOT))
  })
  .bind(&addr)?
  .run()
}

