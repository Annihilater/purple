mod api;
mod app_state;
mod common;
mod config;
mod logging;
mod models;
mod repositories;
mod routes;
mod services;
mod startup;
mod utils;

use startup::Application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let application = Application::build()
        .await
        .expect("Failed to build application");

    application.run().await
}
