use axum::{
    extract::{Extension, Path, Json, ConnectInfo},
    response::{Html, IntoResponse, Json as JsonResponse},
    http::{StatusCode, HeaderMap, HeaderValue},
};
use std::sync::Arc; // For shared state
use std::net::SocketAddr; // For ConnectInfo

use super::constants;
use super::data::Datasources;
use super::data::solver::compute;
use super::entity::{User, PostIdent, CategoryResult, Category};

use log::debug;
use serde_json::json;

use pulldown_cmark::{Parser, Options, html};

// Define a helper type for Axum responses with HTML content type
type HtmlResponse = (HeaderMap, Html<String>);

fn html_content(body: String) -> HtmlResponse {
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::CONTENT_TYPE, HeaderValue::from_static(constants::HTML_CONTENT_TYPE));
    (headers, Html(body))
}

// Define a helper type for Axum responses with JSON content type
type JsonApiResult<T> = (StatusCode, HeaderMap, JsonResponse<T>);

fn json_content<T: serde::Serialize>(status: StatusCode, data: T) -> JsonApiResult<T> {
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::CONTENT_TYPE, HeaderValue::from_static(constants::JSON_CONTENT_TYPE));
    (status, headers, JsonResponse(data))
}

// TODO add proper error handling using Axum's IntoResponse for custom error types

// Removed #[get("/")] macro
// Updated signature: Extension<Arc<Datasources>>, returns impl IntoResponse
pub async fn home(Extension(ds): Extension<Arc<Datasources>>) -> HtmlResponse {
    // Clone config data to ensure correct lifetimes for json! macro
    let site_name = ds.conf().site_domain.clone();
    let data = json!({
        "site_name": site_name,
    });
    // Render first, then create response
    match ds.handlebars().render("home", &data) {
        Ok(body) => html_content(body),
        Err(e) => {
            // Log error, return error message in response
            log::error!("Handlebars render error (home): {}", e);
            html_content(format!("Template error: {}", e))
        }
    }
}

// Removed #[get("/apps")] macro
pub async fn apps(Extension(ds): Extension<Arc<Datasources>>) -> HtmlResponse {
    // Clone config data
    let github_handle = ds.conf().author_github_name.clone();
    let data = json!({
        "apps": {
            "chatapp": "/chatapp" // Assuming this is still relevant
        },
        "github_handle": github_handle
    });
    // Render first, then create response
    match ds.handlebars().render("apps", &data) {
        Ok(body) => html_content(body),
        Err(e) => {
            log::error!("Handlebars render error (apps): {}", e);
            html_content(format!("Template error: {}", e))
        }
    }
}

// Removed #[get("/about")] macro
pub async fn about(Extension(ds): Extension<Arc<Datasources>>) -> HtmlResponse {
    // Clone config data
    let full_name = ds.conf().site_author.clone();
    let data = json!({
        "full_name": full_name
    });
    // Render first, then create response
    match ds.handlebars().render("about", &data) {
        Ok(body) => html_content(body),
        Err(e) => {
            log::error!("Handlebars render error (about): {}", e);
            html_content(format!("Template error: {}", e))
        }
    }
}

// Removed #[get("/contact")] macro
pub async fn contact(Extension(ds): Extension<Arc<Datasources>>) -> HtmlResponse {
    // Clone config data
    let email = ds.conf().author_email.clone();
    let twitter_handle = ds.conf().author_twitter.clone();
    let data = json!({
        "email": email,
        "twitter_handle": twitter_handle
    });
    // Render first, then create response
    match ds.handlebars().render("contact", &data) {
        Ok(body) => html_content(body),
        Err(e) => {
            log::error!("Handlebars render error (contact): {}", e);
            html_content(format!("Template error: {}", e))
        }
    }
}

// Removed #[get("/simple_chat")] macro
pub async fn simple_chat(Extension(ds): Extension<Arc<Datasources>>) -> HtmlResponse {
    let data = json!({}); // Empty data for now
    // Render first, then create response
    match ds.handlebars().render("simple_chat", &data) {
        Ok(body) => html_content(body),
        Err(e) => {
            log::error!("Handlebars render error (simple_chat): {}", e);
            html_content(format!("Template error: {}", e))
        }
    }
}

// Removed #[get("/users")] macro
pub async fn user_list(Extension(ds): Extension<Arc<Datasources>>) -> Result<JsonApiResult<Vec<User>>, StatusCode> {
    // Database operations should ideally be async, or run in a blocking thread pool
    // For simplicity, keeping sync calls here but be aware of blocking risks
    match ds.db().get_users() {
        Ok(users) => Ok(json_content(StatusCode::OK, users)),
        Err(e) => {
            debug!("Failed to get users: {}", e);
            // Consider a more specific error mapping
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
    // Original: 
    /*
    let users: Vec<User> = ds.db().get_users().or_else( |_: rusqlite::Error| -> Result<Vec<User>, String> {
        debug!("No users");
        Ok(Vec::new())
    }).unwrap();
    let body = serde_json::to_string(&users).unwrap(); //format!("{}", json!(users));
    HttpResponse::Ok().content_type(constants::JSON_CONTENT_TYPE).body(body)
    */
}

// Removed #[get("/notes/post/{id}")] macro
// Updated signature: Path extractor for id
pub async fn post_raw(Extension(ds): Extension<Arc<Datasources>>, Path(id): Path<i32>) -> impl IntoResponse {
    // Keeping sync DB call for now
    match ds.db().get_post_by_id(id) {
        Some(post_data) => { // Renamed `data` to `post_data` to avoid conflict
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(&post_data.content, options);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            // Clone data for template
            let title = post_data.ident.title.clone();
            let created = post_data.ident.created.clone(); // Assuming String or similar cloneable type
            let updated = post_data.updated.clone(); // Assuming String or similar cloneable type
            let template_data = json!({
              "raw_post": html_output, // html_output is already owned
              "title": title,
              "created": created,
              "updated": updated
            });
            // Render first, then create response
            match ds.handlebars().render("single_post", &template_data) {
                Ok(body) => Ok(html_content(body)),
                Err(e) => {
                    log::error!("Handlebars render error (single_post): {}", e);
                    // Return error status + message
                    Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)))
                }
            }
        },
        None => {
            // Use tuple for simple status + body response
            Err((StatusCode::NOT_FOUND, "Post Not Found".to_string()))
        }
    }
}

// Removed #[get("/notes")] macro
pub async fn notes(Extension(ds): Extension<Arc<Datasources>>) -> HtmlResponse {
    // Keeping sync DB call for now
    let posts: Vec<PostIdent> = ds.db().get_posts(1000,0).unwrap_or_else(|e| {
        debug!("Failed to get posts: {}", e);
        Vec::new()
    });
    let data = json!({
        "posts": &posts // Reference is okay here as `posts` lives long enough
    });
    // Render first, then create response
    match ds.handlebars().render("notes", &data) {
        Ok(body) => html_content(body),
        Err(e) => {
            log::error!("Handlebars render error (notes): {}", e);
            html_content(format!("Template error: {}", e))
        }
    }
}

// Removed #[get("/utils/whatsmyip")] macro
// Updated signature: Use ConnectInfo extractor for client address
pub async fn whatsmyip(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    // Axum provides the client socket address directly via ConnectInfo
    // Note: This gives the direct peer address. X-Real-IP handling would require reading headers.
    // TODO: Add header reading logic if needed (e.g., using TypedHeader extractor)
    (StatusCode::OK, addr.ip().to_string())
    /* Original:
    let ip_addr_op = req.peer_addr().map(|t| {
        t.ip()
    });
    if let Some(ip_addr) = ip_addr_op {
        let body = match req.headers().get("X-Real-IP") {
            Some(addr) => {
                let ret = match addr.to_str() {
                    Ok(val) => String::from(val),
                    Err(_) => format!("{}", ip_addr)
                };
                ret
            },
            None => format!("{}", ip_addr)
        };
        HttpResponse::Ok().content_type("text/plain").body(body)
    } else {
        HttpResponse::BadRequest().finish()
    }
    */
}

/* // close_db route removed in main.rs as well
#[get("/close_db")]
pub fn close_db(ds: web::Data<Datasources>) -> HttpResponse {
    let op = ds.db().close();
    let body = match op {
        Ok(_) => "DB closed!",
        Err(reason) => reason
    };
    HttpResponse::Ok().content_type("text/plain").body(body.to_string())
}*/

// Updated signature: Json extractor, returns Json response with Vec<Category>
pub async fn solve(Json(payload): Json<Vec<CategoryResult>>) -> JsonResponse<Vec<Category>> {
    // Assuming compute is a sync function for now
    let res = compute(payload);
    JsonResponse(res)
    // Original: HttpResponse::Ok().json(res)
}
