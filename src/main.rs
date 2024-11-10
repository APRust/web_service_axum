#![allow(unused)]

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Router, ServiceExt};
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    // region:    --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();

    // endregion: --- Start Server
}

// region:   --- Handler_hello

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    Html("Hello <strong>World!</strong>")
}

// endregion:   --- Handler_hello
