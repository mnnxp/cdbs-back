pub mod model;
pub(crate) mod service;
// pub mod util;

// use crate::models::relate_ref::file::handler::{
//     add_user,
//     add_component,
//     add_modification,
// };
// use actix_web::web;

// pub fn route(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/files")
//             .service(web::resource("/users").route(web::post().to(add_user)))
//             .service(web::resource("/components/{uuid}").route(web::post().to(add_component)))
//             .service(web::resource("/modifications/{uuid}")
//                 .route(web::post().to(add_modification))),
//     );
// }
