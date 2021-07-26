use actix_web::{http::header, HttpRequest};
use regex::Regex;

lazy_static::lazy_static! {
    static ref BEARER_REGEXP : Regex = Regex::new(r"^Bearer\s(.*)$").expect("Bearer regexp failed!");
}

/// get token from request
pub(crate) fn token_from_request(req: &HttpRequest) -> Option<String> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|authorization| {
            BEARER_REGEXP
                .captures(authorization)
                .and_then(|captures| captures.get(1))
        })
        .map(|v| v.as_str());

    token.map(|t| t.to_string())
}
