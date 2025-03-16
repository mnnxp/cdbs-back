use actix_web::http::header::{HeaderName, HeaderMap, LanguageTag};
use crate::schema::*;
use async_graphql::*;

lazy_static::lazy_static! {
    static ref ACCEPT_LANGUAGE: HeaderName =
        HeaderName::from_lowercase(b"accept-language").unwrap();
}

/// Gets SetLang from request
impl From<&HeaderMap> for SetLang {
    fn from(req: &HeaderMap) -> Self {
        let lang = req
            .get(ACCEPT_LANGUAGE.clone())
            .and_then(|v| v.to_str().ok());

        let lang_id = match lang {
            None => 1,
            Some(str_lang) => {
                // let str_lang = str_lang.parse::<LanguageTag>().unwrap_or_default();

                debug!("ACCEPT_LANGUAGE: {:?}", str_lang);

                // match str_lang.language {
                match str_lang.parse::<LanguageTag>() {
                    // Some(x) if x == *"en" => 1,
                    Ok(x) if x.primary_language() == "ru" => 2,
                    // Some(_) => 1,
                    _ => 1,
                }
            }
        };

        Self { lang_id }
    }
}

// Language models
/// Localization data
#[derive(Debug, Serialize, Deserialize, Queryable, SimpleObject)]
pub(crate) struct Language {
    /// Language identifier (within the platform)
    pub(crate) id: i32,
    /// Full language name
    pub(crate) lang: String,
    /// Abbreviated language name
    pub(crate) langshort: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = language_ref)]
pub(crate) struct InsertableLanguage {
    pub(crate) lang: String,
    pub(crate) langshort: String,
}

/// Data on localization names
#[derive(Debug, Serialize, Deserialize, Queryable, Clone, InputObject)]
pub(crate) struct LanguageData {
    /// Full language name
    pub(crate) lang: String,
    /// Abbreviated language name
    pub(crate) langshort: String,
}

impl From<&LanguageData> for InsertableLanguage {
    fn from(data: &LanguageData) -> Self {
        Self {
            lang: data.lang.clone(),
            langshort: data.langshort.clone(),
        }
    }
}

// for request
pub(crate) struct SetLang {
    pub(crate) lang_id: i32,
}

// for set lang search
pub(crate) struct EngLangName {
    pub(crate) eng_lang_name: String,
}

impl From<&str> for EngLangName {
    fn from(name: &str) -> Self {
        Self {
            eng_lang_name: name.to_string()
        }
    }
}