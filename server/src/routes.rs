
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;

use super::config;

// TODO add proper error handling

#[get("/")]
pub fn home(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "site_name": config::SITE_NAME,
    });
    let body = hb.render("home", &data).unwrap();

    HttpResponse::Ok().content_type(config::HTML_CONTENT_TYPE).body(body)
}

#[get("/apps")]
pub fn apps(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "apps": {
            "chatapp": "/chatapp"
        },
        "github_handle": config::GITHUB_HANDLE
    });
    let body = hb.render("apps", &data).unwrap();
    HttpResponse::Ok().content_type(config::HTML_CONTENT_TYPE).body(body)
}

#[get("/about")]
pub fn about(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "full_name": config::MY_NAME
    });
    let body = hb.render("about", &data).unwrap();
    HttpResponse::Ok().content_type(config::HTML_CONTENT_TYPE).body(body)
}

#[get("/contact")]
pub fn contact(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "email": config::MY_EMAIL,
        "twitter_handle": config::MY_TWITTER_HANDLE
    });
    let body = hb.render("contact", &data).unwrap();
    HttpResponse::Ok().content_type(config::HTML_CONTENT_TYPE).body(body)
}
