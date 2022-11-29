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

use std::{net::SocketAddr, sync::Arc};

use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::{Extension, Router, Server};
use color_eyre::Result;
use state::{make_services, Dependencies, Services};

#[macro_use]
extern crate tracing;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    color_eyre::install()?;

    info!("hello");
    let addr = addr().unwrap_or_else(|_| ([0, 0, 0, 0], 8000).into());

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
    let mut api = OpenApi::default();
    let router = ApiRouter::new();
    let router = routing::attach_routes(router);
    router
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)))
        .with_state(state)
}

static_assertions::assert_impl_all!(Router<()>: Clone);

fn addr() -> Result<SocketAddr> {
    let s = std::env::var("BIND_ADDR")?;
    Ok(s.parse()?)
}
