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
mod schema;
mod storage;

use actix_cors::Cors;
use actix_web::middleware::Logger;
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

    // Database
    let pool = database::pool::establish_connection(opt.clone());

    // Server port
    let domain = opt.domain.clone();
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
            .app_data(Data::new(pool.clone()))
            // .app_data(schema)
            .app_data(Data::new(schema.clone()))
            // Options
            .app_data(Data::new(opt.clone()))
            // CORS
            .wrap(cors)
            // Error logging
            .wrap(Logger::default())
            // Sets routes via secondary files
            .configure(models::user::route)
            .configure(graphql::route)
    })
    // Running at `format!("{}:{}",port,"0.0.0.0")`
    .bind((domain.clone(), port))
    .unwrap()
    // Starts server
    .run();

    eprintln!("Listening on {domain}:{port}");

    // Awaiting server to exit
    server.await
}
