
use actix_web::{web, HttpResponse};

use super::constants;
use super::data::Datasources;
use super::entity::{User, PostIdent};

use log::{debug};
use serde_json::json;

use pulldown_cmark::{Parser, Options, html};

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

#[get("/echo")]
pub fn echochat(ds: web::Data<Datasources>) -> HttpResponse {
    let data = json!({"a": "b"});
    let body = ds.handlebars().render("echo", &data).unwrap();
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

#[get("/blog/post/{id}")]
pub fn post_raw(ds: web::Data<Datasources>, info: web::Path<i32>) -> HttpResponse {
    let post = ds.db().get_post_by_id(info.into_inner());
    match post {
        Some(data) => {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(&data.content, options);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            //HttpResponse::Ok().content_type("text/plain").body(data)
            let data = json!({
              "raw_post": &html_output,
              "title": &data.ident.title,
              "created": &data.ident.created,
              "updated": &data.updated
            });
            let body = ds.handlebars().render("single_post", &data).unwrap();
            HttpResponse::Ok().content_type(constants::HTML_CONTENT_TYPE).body(body)
        },
        None => {
            HttpResponse::NotFound().body("Post Not Found".to_string())
        }
    }
}

#[get("/blog")]
pub fn blog(ds: web::Data<Datasources>) -> HttpResponse {
    // TODO get offset and limit from the request query params...
    // current limit: 1000 posts which I won't reach anytime soon.
    let posts: Vec<PostIdent> = ds.db().get_posts(1000,0).or_else(|_: rusqlite::Error| -> Result<Vec<PostIdent>, String> {
        debug!("No posts");
        Ok(Vec::new())
    }).unwrap();
    let data = json!({
        "posts": &posts
    });
    let body = ds.handlebars().render("blog", &data).unwrap();
    HttpResponse::Ok().content_type(constants::HTML_CONTENT_TYPE).body(body)
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
