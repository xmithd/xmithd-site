
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;

use super::config;

#[get("/")]
pub fn home(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "site_name": config::SITE_NAME
    });
    let body = hb.render("home", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/apps")]
pub fn apps(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "apps": {
            "chatapp": "/chatapp"
        }
    });
    let body = hb.render("apps", &data).unwrap();
    HttpResponse::Ok().body(body)
}
