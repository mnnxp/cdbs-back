use actix_web::{web, HttpRequest, HttpResponse, Result};
use async_graphql::http::playground_source;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use crate::cli_args::Opt;
use crate::database::Pool;
use crate::graphql::{MutationRoot, QueryRoot};
use crate::jwt::model::Token;
use crate::models::relate_ref::language::model::SetLang;

type ActixSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn build_schema(pool: Pool) -> ActixSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription
    )
        .enable_federation()
        .data(pool)
        .finish()
}

pub async fn graphql(
    schema: web::Data<ActixSchema>,
    req: HttpRequest,
    gql_request: Request,
) -> Response {
    let mut request = gql_request.into_inner();

    let headers_req = req.headers();

    // get token from request
    let token: Token = headers_req.into();

    // set the language for sending responses
    let lang: SetLang = headers_req.into();

    // println!("match token Ok");
    request = request.data(token);
    request = request.data(lang);

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
