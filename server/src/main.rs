extern crate actix_web;

use std::{io};

use actix_files as fs;
use actix_web::{
  web, App, HttpServer,
  Result,
};

fn render_file(file_name: &str) -> Result<fs::NamedFile> {
  let file = fs::NamedFile::open(file_name)?;
  Ok(file)
}

fn ui_app() -> Result<fs::NamedFile> {
  render_file("../ui/build/index.html")
}

fn home_page() -> Result<fs::NamedFile> {
  render_file("./templates/home.html")
}

fn favicon() -> Result<fs::NamedFile> {
  render_file("./templates/favicon.ico")
}

fn main() -> io::Result<()> {
  let port: u32 = 3001;
  let addr = format!("127.0.0.1:{}", port);
  println!("Starting http server at http://{}", &addr);

  HttpServer::new(|| {
    App::new()
    .service(web::resource("/").route(web::get().to(home_page)))
    .service(web::resource("/favicon.ico").route(web::get().to(favicon)))
    .service(web::resource("/webapp").route(web::get().to(ui_app)))
    .service(fs::Files::new("/webapp", "../ui/build"))
  })
  .bind(&addr)?
  .run()
}

