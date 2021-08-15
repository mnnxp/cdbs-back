use actix_web::{web, HttpRequest, HttpResponse, Result};
use async_graphql::http::playground_source;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use crate::cli_args::Opt;
use crate::database::Pool;
use crate::graphql::{mutations::MutationRoot, queries::QueryRoot};
use crate::jwt::model::Token;
use crate::models::language::model::SetLang;

type ActixSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn build_schema(pool: Pool) -> ActixSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .enable_federation()
        .data(pool)
        .finish()
}

pub async fn graphql(
    schema: web::Data<ActixSchema>,
    req: HttpRequest,
    gql_request: Request,
    // user: LoggedUser,
    // token: DecodedToken,
    // pool: web::Data<Pool>,
    // opt: web::Data<Opt>
) -> Response {
    let mut request = gql_request.into_inner();

    let token: Token = req.into();

    // use actix_web::{http::header, HttpRequest};
    // use regex::Regex;
    // let re_lang = Regex::new(r"^[1-2]$").expect("Lang regexp failed!");
    // let result = req
    //     .headers()
    //     .get(header::HeaderName::from_lowercase(b"language").unwrap())
    //     .and_then(|v| v.to_str().ok())
    //     .and_then(|language| {
    //         // if two language
    //         re_lang.find(language).unwrap().range().
    //     });
    let lang: SetLang = SetLang { id_lang: 1 };

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
