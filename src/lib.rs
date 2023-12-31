use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use crate::routes::{health_check, subscribe};

pub mod configuration;
pub mod routes;
pub mod startup;


// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    }).listen(listener)? .run();
// No .await here!
    Ok(server)
}