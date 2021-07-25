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

// pub use util::has_supplier;
pub(crate) use util::hash_authorized;
pub(crate) use util::get_uuid_user;
// pub use util::verify_uuid_user;
