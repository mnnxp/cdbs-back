mod handler;
pub mod model;
pub(crate) mod service;
pub mod util;

use crate::models::user_represent::handler::{register, delete};
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/represents")
            .service(web::resource("").route(web::post().to(register)))
            .service(web::resource("/{uuid_represent}").route(web::delete().to(delete))),
    );
}
