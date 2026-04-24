use crate::cli_args::Opt;
use crate::database::Pool;
use crate::graphql::{MutationRoot, QueryRoot};
use crate::auth::jwt::model::Token;
use crate::models::relate_ref::language::model::SetLang;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_web::http::header::{HeaderMap, HOST, ORIGIN};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema, Context
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

type ActixSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn build_schema(pool: Pool) -> ActixSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    // .enable_federation()
    .data(pool)
    .finish()
}

pub async fn graphql(
    schema: web::Data<ActixSchema>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();

    let headers_req = req.headers();

    // get token from request
    let token: Token = headers_req.into();

    // set the language for sending responses
    let lang: SetLang = headers_req.into();

    // extract the client's domain from HTTP headers
    let domain: ClientDomain = headers_req.into();

    // println!("match token Ok");
    request = request.data(token);
    request = request.data(lang);
    request = request.data(domain);

    schema.execute(request).await.into()
}

pub async fn graphiql(opt: web::Data<Opt>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new(&opt.api_point).subscription_endpoint(&opt.api_point),
        )))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientDomain {
    pub domain: String,
}

/// get token from request
impl From<&HeaderMap> for ClientDomain {
    fn from(req: &HeaderMap) -> Self {
        // Try different headers in order
        let domain = req
            .get(HOST) // The most reliable one
            .or_else(|| req.get(ORIGIN))
            .or_else(|| req.get("X-Forwarded-Host")) // If behind a proxy
            .and_then(|h| h.to_str().ok())
            .map(|s| {
                // Remove protocol and port
                s.replace("https://", "")
                .replace("http://", "")
                .split(':')
                .next()
                .unwrap_or(s)
                .to_string()
            }).unwrap_or_default();
        debug!("Referer: {}", domain);
        Self { domain }
    }
}

/// Extract the client's domain from HTTP headers
pub(crate) fn extract_client_domain(cxt: &Context<'_>) -> String {
    match cxt.data_opt::<ClientDomain>() {
        Some(cd) => cd.domain.clone(),
        None => "unknown".to_string(),
    }
}