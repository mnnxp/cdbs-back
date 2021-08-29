#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod cli_args;
mod database;
mod errors;
mod graphql;
mod jwt;
mod models;
mod storage;
mod schema;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

// use crate::graphql::handler::{graphiql, graphql};
// use crate::graphql::{mutations::MutationRoot, queries::QueryRoot};
// use actix_web::{guard, web};
// use async_graphql::EmptySubscription;

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

    // Database
    let pool = database::pool::establish_connection(opt.clone());

    // Server port
    let port = opt.port;

    let schema = crate::graphql::handler::build_schema(pool.clone()).await;

    // Server
    let server = HttpServer::new(move || {
        // prevents double Arc
        // let schema: web::Data<graphql::model::Schema> = schema.clone().into();
        // CORS a very permissive set of default for quick development
        let cors = Cors::permissive();
        App::new()
            // Database
            .data(pool.clone())
            // .app_data(schema)
            .data(schema.clone())
            // Options
            .data(opt.clone())
            // CORS
            .wrap(cors)
            // Error logging
            .wrap(Logger::default())
            // Authorisation (now we do not use cookies)
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(cookie_secret_key.as_bytes())
            //         .name("auth")
            //         .path("/")
            //         .domain(&domain)
            //         // Time from creation that cookie remains valid
            //         .max_age_time(auth_duration)
            //         // Restricted to https?
            //         .secure(secure_cookie),
            // ))
            // Sets routes via secondary files
            .configure(models::user::route)
            .configure(graphql::route)
            // .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
            // .service(web::resource("/").guard(guard::Get()).to(graphiql))
    })
    // Running at `format!("{}:{}",port,"0.0.0.0")`
    .bind(("0.0.0.0", port))
    .unwrap()
    // Starts server
    .run();

    eprintln!("Listening on 0.0.0.0:{}", port);

    // Awaiting server to exit
    server.await
}
