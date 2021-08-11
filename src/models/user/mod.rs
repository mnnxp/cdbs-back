mod handler;
pub mod model;
pub(crate) mod notification;
pub(crate) mod service;
pub(crate) mod certificate;
pub(crate) mod util;

use crate::models::user::handler::login;
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/login").route(web::post().to(login)));
}

pub(crate) use util::*;
