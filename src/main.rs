#![warn(clippy::all)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![forbid(unsafe_code)]

mod config;
mod db;
mod model;
mod routing;
mod state;

#[cfg(test)]
mod testing;

use std::net::SocketAddr;

use axum::{Router, Server};
use color_eyre::Result;
use state::{make_services, Dependencies, Services};

#[macro_use]
extern crate tracing;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    color_eyre::install()?;

    info!("hello");
    let addr = addr().unwrap_or_else(|_| ([127, 0, 0, 1], 8000).into());

    let deps = Dependencies::load()?;
    let services = make_services(deps)?;

    Server::bind(&addr)
        .serve(make_app(services).into_make_service())
        .await
        .unwrap();

    info!("goodbye");
    Ok(())
}

fn make_app(state: Services) -> Router<()> {
    let router = Router::new();
    let router = routing::attach_routes(router);
    router.with_state(state)
}

fn addr() -> Result<SocketAddr> {
    let s = std::env::var("BIND_ADDR")?;
    Ok(s.parse()?)
}
