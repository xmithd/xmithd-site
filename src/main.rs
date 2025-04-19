mod constants;
mod routes;
mod data;
mod entity;

use std::sync::Arc;
use std::net::SocketAddr;

extern crate log;
extern crate handlebars;

use log::{info, error};

use axum::{
    routing::{get, post},
    Router,
    extract::Extension,
};
use tokio::net::TcpListener;
use tower_http::{
    trace::{DefaultMakeSpan, TraceLayer},
    services::ServeDir,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "xmithd_backend=debug,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

  let state = data::Datasources::new();
  let addr_str = format!("{}:{}", state.conf().host, state.conf().port);
  let addr: SocketAddr = addr_str.parse()?;

  info!("Starting http server at http://{}", &addr_str);
  let static_files_path = String::from(&state.conf().static_files);
  let datasources_arc = Arc::new(state);

  let app = Router::new()
      .route("/", get(routes::home))
      .route("/apps", get(routes::apps))
      .route("/about", get(routes::about))
      .route("/contact", get(routes::contact))
      .route("/notes", get(routes::notes))
      .route("/notes/post/{id}", get(routes::post_raw))
      .route("/users", get(routes::user_list))
      .route("/simple_chat", get(routes::simple_chat))
      .route("/utils/whatsmyip", get(routes::whatsmyip))
      .route("/api/inventory/solve", post(routes::solve))
      .nest_service("/public", ServeDir::new(&static_files_path))
      .fallback_service(ServeDir::new(constants::PUBLIC_FOLDER))
      .layer(Extension(datasources_arc.clone()))
      .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
      );

  info!("listening on {}", addr);
  let listener = TcpListener::bind(addr).await?;
  axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

  // Attempt to get the Datasources back from the Arc
  let close_result = match Arc::try_unwrap(datasources_arc) {
      Ok(ds) => {
          info!("Server shut down gracefully. Closing database.");
          ds.close_db()
              .map_err(|e| {
                  error!("Error closing database: {}", e);
                  // Convert rusqlite::Error to Box<dyn std::error::Error>
                  Box::new(e) as Box<dyn std::error::Error>
              })
      },
      Err(_) => {
          // This should theoretically not happen if the server shut down cleanly
          error!("Failed to get exclusive ownership of Datasources; DB not closed explicitly.");
          Ok(()) // Return Ok if we couldn't unwrap, not a critical error for main
      }
  };

  close_result // Return the result from the match
}

