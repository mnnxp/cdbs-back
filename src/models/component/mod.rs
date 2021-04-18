mod handler;
pub mod model;
pub(crate) mod service;
pub mod util;

use crate::models::component::handler::register;
use crate::models::param::handler::add_to_component;
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/components")
            .service(web::resource("").route(web::post().to(register)))
            .service(web::resource("/params")
                .route(web::post().to(add_to_component))),
    );
}
