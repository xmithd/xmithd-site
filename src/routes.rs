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
pub async fn home(Extension(ds): Extension<Arc<Datasources>>) -> HtmlResponse {
    // Clone config data to ensure correct lifetimes for json! macro
    let site_name = &ds.conf().site_domain;
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
    let full_name = &ds.conf().site_author;
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
    let email = &ds.conf().author_email;
    let twitter_handle = &ds.conf().author_twitter;
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
}

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
            let title = &post_data.ident.title;
            let created = post_data.ident.created;
            let updated = post_data.updated;
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

// Use ConnectInfo extractor for client address and HeaderMap
pub async fn whatsmyip(ConnectInfo(addr): ConnectInfo<SocketAddr>, headers: HeaderMap) -> impl IntoResponse {
    // Axum provides the client socket address directly via ConnectInfo
    // Check for X-Real-IP header first
    let ip_to_return = headers
        .get("X-Real-IP")
        .and_then(|hv| hv.to_str().ok())
        .map(|s| s.to_string()) // Convert valid header to String
        .unwrap_or_else(|| addr.ip().to_string()); // Fallback to peer IP

    (StatusCode::OK, ip_to_return)
}

// Updated signature: Json extractor, returns Json response with Vec<Category>
pub async fn solve(Json(payload): Json<Vec<CategoryResult>>) -> JsonResponse<Vec<Category>> {
    // Assuming compute is a sync function for now
    let res = compute(payload);
    JsonResponse(res)
}
