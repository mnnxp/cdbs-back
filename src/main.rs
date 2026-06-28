#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod cli_args;
mod config;
mod database;
mod errors;
mod graphql;
mod auth;
mod models;
mod schema;
mod storage;
mod macros;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::http::header;
use actix_web::{web::Data, App, HttpServer};
use crate::database::pool::establish_connection;
use crate::graphql::handler::build_schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Gets enviroment variables from `.env`
    dotenv::dotenv().ok();

    // Initiates error logger
    env_logger::init();

    // Sets options to enviroment variables
    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };
    let opt_data = Data::new(opt.clone());

    // Database
    let pool = establish_connection(opt.clone());
    let pool_data = Data::new(pool.clone());
    let schema = Data::new(build_schema(pool).await);

    // Server port
    let domain = opt.domain.clone();
    let port = opt.port;

    // Parse allowed origins from comma-separated env var
    let allowed_origins: Vec<String> = opt.allowed_origins
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // Server
    let server = HttpServer::new(move || {
        let origins = allowed_origins.clone();
        // Configure CORS: restrict origins, methods, and headers
        let cors = Cors::default()
            .allowed_origin_fn(move |origin, _req| {
                origins.iter().any(|allowed| origin == allowed)
            })
            .allowed_methods(vec!["OPTIONS", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
                header::ACCEPT_LANGUAGE,
            ])
            .supports_credentials()
            .max_age(3600);
        // Build
        App::new()
            // CORS
            .wrap(cors)
            // Error logging
            .wrap(Logger::default())
            // Options
            .app_data(opt_data.clone())
            // Database
            .app_data(pool_data.clone())
            // .app_data(schema)
            .app_data(schema.clone())
            // Sets routes via secondary files
            .configure(models::user::route)
            .configure(graphql::route)
    })
    .bind((domain, port))
    .unwrap()
    // Starts server
    .run();

    eprintln!("Listening on {}:{}", opt.domain, opt.port);

    // Awaiting server to exit
    server.await
}
