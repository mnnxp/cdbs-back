pub(crate) mod handler;
pub mod queries;
pub mod mutations;

use actix_web::{guard, web};
use crate::graphql::handler::{graphql, graphiql};

pub(super) fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").guard(guard::Post()).to(graphql))
        .service(web::resource("/").guard(guard::Get()).to(graphiql));
}
