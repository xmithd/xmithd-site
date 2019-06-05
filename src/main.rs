extern crate actix_web;

use std::{io};

use actix_files as fs;
use actix_web::{
  web, App, HttpServer,
  Result,
};

fn index() -> Result<fs::NamedFile> {
  let file = fs::NamedFile::open("ui/build/index.html")?;
  Ok(file)
}

fn main() -> io::Result<()> {
  let port: u32 = 3001;
  let addr = format!("127.0.0.1:{}", port);
  println!("Starting http server at http://{}", &addr);

  HttpServer::new(|| {
    App::new()
    .service(web::resource("/").route(web::get().to(index)))
    .service(fs::Files::new("/", "ui/build"))
  })
  .bind(&addr)?
  .run()
}

