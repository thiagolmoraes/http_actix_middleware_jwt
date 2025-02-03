use actix_web::{HttpServer, App};
use anyhow::{Context, Result};

// Importing modules to be used
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod services;

use handlers::init_routes;

#[actix_web::main]
async fn main() -> Result<()> {


    HttpServer::new( move || {
        App::new()
           .wrap(middleware::Authentication)
           .configure(init_routes)
    })
    .bind("127.0.0.1:8000")
    .context("Failed to bind")?
    .run()
    .await
    .context("Failed to start server")?;

    Ok(())

}