use crate::schema::*;
use async_graphql::*;
// use chrono::*;

// Language models

#[derive(Debug, Serialize, Deserialize, Queryable, SimpleObject)]
pub struct Language {
    pub id: i32,
    pub lang: String,
    pub langshort: String,
}

#[derive(Debug, Insertable)]
#[table_name = "language_ref"]
pub struct InsertableLanguage {
    pub lang: String,
    pub langshort: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, InputObject)]
pub struct LanguageData {
    pub lang: String,
    pub langshort: String,
}

impl From<LanguageData> for InsertableLanguage {
    fn from(ipt_data: LanguageData) -> Self {
        let LanguageData {
            lang,
            langshort,
            ..
        } = ipt_data;

        Self {
            lang,
            langshort,
        }
    }
}

use actix_web::http::{HeaderName, HeaderMap, header::LanguageTag};
// use actix_web::http::{HeaderName, HeaderMap};
lazy_static::lazy_static! {
    static ref ACCEPT_LANGUAGE: HeaderName =
        HeaderName::from_lowercase(b"accept-language").unwrap();
}

/// get SetLang from request
impl From<&HeaderMap> for SetLang {
    fn from(req: &HeaderMap) -> Self {
        let lang = req
            .get(ACCEPT_LANGUAGE.clone())
            .and_then(|v| v.to_str().ok());

        let lang_id = match lang {
            None => 1,
            Some(str_lang) => {
                let str_lang = str_lang.parse::<LanguageTag>().unwrap_or_default();

                debug!("ACCEPT_LANGUAGE: {:?}", str_lang);

                match str_lang.language {
                    // Some(x) if x == *"en" => 1,
                    Some(x) if x == *"ru" => 2,
                    // Some(_) => 1,
                    _ => 1,
                }
            }
        };

        Self { lang_id }
    }
}

// for request
pub struct SetLang {
    pub lang_id: i32,
}

// for set lang search
pub struct EngLangName {
    pub eng_lang_name: String,
}

impl From<&str> for EngLangName {
    fn from(name: &str) -> Self {
        Self {
            eng_lang_name: name.to_string()
        }
    }
}
