// use std::sync::Arc;

// use crate::graphql::model::Context;
// use crate::jwt::model::DecodedToken;
// use crate::models::user::model::LoggedUser;
use actix_web::{web, HttpRequest, HttpResponse, Result};
// use actix_web::{error, Error};

use async_graphql::http::playground_source;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::{EmptySubscription, Schema};
// use async_graphql::SchemaBuilder;
use async_graphql_actix_web::{Request, Response};
// use async_graphql_actix_web::{BatchRequest, Request, Response};

use crate::cli_args::Opt;
use crate::database::Pool;
// use crate::database::{db_connection, Pool};
use crate::graphql::{mutations::MutationRoot, queries::QueryRoot};

type ActixSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub struct MyToken(pub String);

pub async fn build_schema(pool: Pool) -> ActixSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .enable_federation()
        .data(pool)
        .finish()
}

// pub fn build_schema() -> ActixSchema  {
//   Schema::new(QueryRoot, MutationRoot, EmptySubscription)
// }

pub async fn graphql(
    schema: web::Data<ActixSchema>,
    req: HttpRequest,
    gql_request: Request,
    // user: LoggedUser,
    // token: DecodedToken,
    // pool: web::Data<Pool>,
    // opt: web::Data<Opt>
) -> Response {
    let token = req
        .headers()
        .get("Token")
        .and_then(|value| value.to_str().map(|s| MyToken(s.to_string())).ok());
    let mut request = gql_request.into_inner();
    if let Some(token) = token {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

pub async fn graphiql(opt: web::Data<Opt>) -> Result<HttpResponse> {
    let gql_ver = &format!("http://{}:{}/graphql", opt.domain, opt.port);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new(gql_ver).subscription_endpoint(gql_ver),
        )))
}
