mod handler;
pub mod model;
pub(crate) mod service;
pub mod util;

use crate::models::user_represet::handler::{register, delete };
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/represets")
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/delete").route(web::post().to(delete))),
    );
}
