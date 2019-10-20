
use actix_web::{web, HttpResponse};

use super::constants;
use super::data::Datasources;
use super::entity::User;

use log::{debug};
use serde_json::json;

// TODO add proper error handling

#[get("/")]
pub fn home(ds: web::Data<Datasources>) -> HttpResponse {
    let data = json!({
        "site_name": &ds.conf().site_domain,
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
        "github_handle": &ds.conf().author_github_name
    });
    let body = ds.handlebars().render("apps", &data).unwrap();
    HttpResponse::Ok().content_type(constants::HTML_CONTENT_TYPE).body(body)
}

#[get("/about")]
pub fn about(ds: web::Data<Datasources>) -> HttpResponse {
    let data = json!({
        "full_name": &ds.conf().site_author
    });
    let body = ds.handlebars().render("about", &data).unwrap();
    HttpResponse::Ok().content_type(constants::HTML_CONTENT_TYPE).body(body)
}

#[get("/contact")]
pub fn contact(ds: web::Data<Datasources>) -> HttpResponse {
    let data = json!({
        "email": &ds.conf().author_email,
        "twitter_handle": &ds.conf().author_twitter
    });
    let body = ds.handlebars().render("contact", &data).unwrap();
    HttpResponse::Ok().content_type(constants::HTML_CONTENT_TYPE).body(body)
}

#[get("/users")]
pub fn user_list(ds: web::Data<Datasources>) -> HttpResponse {
    let users: Vec<User> = ds.db().get_users().or_else( |_: rusqlite::Error| -> Result<Vec<User>, String> {
        debug!("No users");
        Ok(Vec::new())
    }).unwrap();
    let body = serde_json::to_string(&users).unwrap(); //format!("{}", json!(users));
    HttpResponse::Ok().content_type(constants::JSON_CONTENT_TYPE).body(body)
}

/*#[get("/close_db")]
pub fn close_db(ds: web::Data<Datasources>) -> HttpResponse {
    let op = ds.db().close();
    let body = match op {
        Ok(_) => "DB closed!",
        Err(reason) => reason
    };
    HttpResponse::Ok().content_type("text/plain").body(body.to_string())
}*/
