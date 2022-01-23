mod handler;
pub(crate) mod access;
pub(crate) mod model;
pub(crate) mod notification;
pub(crate) mod service;
pub(crate) mod relate;
pub(crate) mod repository;
pub(crate) mod util;

pub(crate) use relate::*;

use crate::models::user::handler::login;
use actix_web::web;

pub(crate) fn route(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/login").route(web::post().to(login)));
}

// pub(crate) use util::*;
