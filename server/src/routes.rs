
use actix_web::{web, HttpResponse};

use super::constants;
use super::data::Datasources;

// TODO add proper error handling

#[get("/")]
pub fn home(ds: web::Data<Datasources>) -> HttpResponse {
    let data = json!({
        "site_name": constants::SITE_NAME,
    });
    let body = ds.handlebars().render("home", &data).unwrap();

    HttpResponse::Ok().content_type(constants::HTML_CONTENT_TYPE).body(body)
}

#[get("/apps")]
pub fn apps(ds: web::Data<Datasources>) -> HttpResponse {
    let data = json!({
        "apps": {
            "chatapp": "/chatapp"
        },
        "github_handle": constants::GITHUB_HANDLE
    });
    let body = ds.handlebars().render("apps", &data).unwrap();
    HttpResponse::Ok().content_type(constants::HTML_CONTENT_TYPE).body(body)
}

#[get("/about")]
pub fn about(ds: web::Data<Datasources>) -> HttpResponse {
    let data = json!({
        "full_name": constants::MY_NAME
    });
    let body = ds.handlebars().render("about", &data).unwrap();
    HttpResponse::Ok().content_type(constants::HTML_CONTENT_TYPE).body(body)
}

#[get("/contact")]
pub fn contact(ds: web::Data<Datasources>) -> HttpResponse {
    let data = json!({
        "email": constants::MY_EMAIL,
        "twitter_handle": constants::MY_TWITTER_HANDLE
    });
    let body = ds.handlebars().render("contact", &data).unwrap();
    HttpResponse::Ok().content_type(constants::HTML_CONTENT_TYPE).body(body)
}
