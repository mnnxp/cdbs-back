mod handler;
pub mod model;
pub(crate) mod notification;
pub(crate) mod service;
pub(crate) mod util;

use crate::models::user::handler::login;
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(web::resource("/login").route(web::post().to(login)))
            // .service(web::resource("/logout").route(web::get().to(logout)))
            // .service(web::resource("/me").route(web::get().to(me))),
    );
}

pub(crate) use util::*;
