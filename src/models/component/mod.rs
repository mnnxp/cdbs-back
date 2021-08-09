pub mod model;
pub(crate) mod component_modification;
pub(crate) mod license;
pub(crate) mod param;
pub(crate) mod service;
// pub mod util;

// use crate::models::component::handler::register;
// use crate::models::param::handler::add_component_param;
// use actix_web::web;
//
// pub fn route(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/components")
//             .service(web::resource("").route(web::post().to(register)))
//             .service(web::resource("/params")
//                 .route(web::post().to(add_component_param))),
//     );
// }
