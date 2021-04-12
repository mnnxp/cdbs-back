mod handler;
pub mod model;
pub(crate) mod service;
pub mod util;

use crate::models::param::handler::{
    register,
    add_to_component,
    add_to_modification
};
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/param")
            .service(web::resource("/add").route(web::post().to(register)))
            .service(web::resource("/component")
                .route(web::post().to(add_to_component)))
            .service(web::resource("/modification")
                .route(web::post().to(add_to_modification))),
    );
}
