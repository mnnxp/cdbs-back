use std::sync::Arc;

use crate::graphql::model::Context;
use crate::jwt::model::DecodedToken;
use crate::models::user::model::LoggedUser;
use actix_web::{error, web, Error, HttpRequest, HttpResponse, Result};

use async_graphql::http::playground_source;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
use async_graphql_actix_web::{BatchRequest, Request, Response};

use crate::cli_args::Opt;
use crate::database::{db_connection, Pool};
use crate::graphql::{mutations::MutationRoot, queries::QueryRoot};

type ActixSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

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
    schema.execute(gql_request.into_inner()).await.into()

    // let db_pool = db_connection(&pool)?;

    // let opt = opt.into_inner().as_ref().clone();
    // let ctx = Context::new(token, user, db_pool, opt);

    // let db_pool = db_connection(&pool)?;

    // let opt = opt.into_inner().as_ref().clone();
    // let ctx = Context::new(token, user, db_pool, opt);

    // let res = web::Json(Response(st.execute(data.into_inner()).await)).execute(&st, &ctx);
    // let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

    // Ok(HttpResponse::Ok()
    //     .content_type("application/json")
    //     .body(json))
}

pub async fn graphiql(opt: web::Data<Opt>) -> Result<HttpResponse> {
    let gql_ver = &format!("http://{}:{}/graphql", opt.domain, opt.port);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new(gql_ver).subscription_endpoint(gql_ver),
        )))
}
