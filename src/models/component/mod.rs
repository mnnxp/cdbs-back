mod handler;
pub mod model;
pub(crate) mod service;
pub mod util;

use crate::models::component::handler::{register};
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/component")
            .service(web::resource("/add").route(web::post().to(register))),
    );
}
