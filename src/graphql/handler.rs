use actix_web::{error, web, Error, HttpResponse, Result};
use crate::graphql::model::Context;
use crate::jwt::model::DecodedToken;
use crate::models::user::model::LoggedUser;

use async_graphql::http::playground_source;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::Request;

use crate::cli_args::Opt;
use crate::database::{db_connection, Pool};
use crate::graphql::{queries::QueryRoot, mutations::MutationRoot};


type ActixSchema = Schema<
    QueryRoot,
    MutationRoot,
    EmptySubscription,
>;

pub async fn build_schema() -> ActixSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}

pub async fn graphql(
    st: web::Data<ActixSchema>,
    data: web::Json<Request>,
    user: LoggedUser,
    token: DecodedToken,
    pool: web::Data<Pool>,
    opt: web::Data<Opt>,
) -> Result<HttpResponse, Error> {
    let db_pool = db_connection(&pool)?;

    let opt = opt.into_inner().as_ref().clone();
    let ctx = Context::new(token, user, db_pool, opt);

    let res = data.execute(&st, &ctx);
    let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

pub async fn graphiql(opt: web::Data<Opt>) -> Result<HttpResponse> {
    let gql_ver = &format!("http://{}:{}/graphql", opt.domain, opt.port);
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        playground_source(
            GraphQLPlaygroundConfig::new(gql_ver).subscription_endpoint(gql_ver),
        ),
    ))
}
