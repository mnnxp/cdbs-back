#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod auth;
mod cli_args;
mod config;
mod database;
mod errors;
mod graphql;
mod macros;
mod models;
mod schema;
mod storage;

use crate::auth::middleware::AuthMiddleware;
use crate::auth::token::manager::init_jwt_keys;
use crate::config::init_config;
use crate::database::pool::establish_connection;
use crate::graphql::handler::build_schema;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{web::Data, App, HttpServer};

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

    // Freeze application configuration globally
    if let Err(err) = init_config(opt.clone()) {
        log::error!("Configuration error: {}", err);
        std::process::exit(1);
    }

    if let Err(err) = init_jwt_keys(&opt.jwt_private_key, &opt.jwt_public_key) {
        log::error!("Invalid JWT RSA keys format: {:?}", err);
        std::process::exit(1);
    }

    // Database
    let pool = establish_connection();
    let pool_data = Data::new(pool.clone());
    let schema = Data::new(build_schema(pool).await);

    // Server port
    let domain = opt.domain.clone();
    let port = opt.port;

    // Parse allowed origins from comma-separated env var
    let allowed_origins: Vec<String> = opt
        .allowed_origins
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // Server
    let server = HttpServer::new(move || {
        let origins = allowed_origins.clone();
        // Configure CORS: restrict origins, methods, and headers
        let cors = Cors::default()
            .allowed_origin_fn(move |origin, _req| origins.iter().any(|allowed| origin == allowed))
            .allowed_methods(vec!["OPTIONS", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
                header::ACCEPT_LANGUAGE,
                header::HeaderName::from_static("x-api-key"),
            ])
            .supports_credentials()
            .max_age(3600);

        // Configure security headers
        let security_headers = DefaultHeaders::new()
            .add(("Cross-Origin-Opener-Policy", "same-origin"))
            .add(("Cross-Origin-Embedder-Policy", "require-corp"))
            .add(("X-Content-Type-Options", "nosniff"))
            .add(("X-Frame-Options", "DENY"))
            // Enforces Strict HTTPS (HSTS) for 30 days
            .add((
                "Strict-Transport-Security",
                "max-age=2592000; includeSubDomains",
            ));

        // Build
        App::new()
            // CORS
            .wrap(cors)
            // COOP, COEP and OWASP
            .wrap(security_headers)
            // Error logging
            .wrap(Logger::default())
            // Authentication
            .wrap(AuthMiddleware)
            // Database
            .app_data(pool_data.clone())
            // .app_data(schema)
            .app_data(schema.clone())
            // Sets routes via secondary files
            .configure(models::user::route)
            .configure(graphql::route)
    })
    .bind((domain, port))
    .expect("Failed to bind to socket")
    // Starts server
    .run();

    eprintln!("Listening on {}:{}", opt.domain, opt.port);

    // Awaiting server to exit
    server.await
}
