use actix_web::http::header::{HeaderName, HeaderMap, LanguageTag};
use crate::schema::*;
use async_graphql::*;

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

#[derive(Debug, Serialize, Deserialize, Queryable, SimpleObject)]
pub(crate) struct Language {
    pub(crate) id: i32,
    pub(crate) lang: String,
    pub(crate) langshort: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = language_ref)]
pub(crate) struct InsertableLanguage {
    pub(crate) lang: String,
    pub(crate) langshort: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, InputObject)]
pub(crate) struct LanguageData {
    pub(crate) lang: String,
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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptLanguageArg {
    pub(crate) lang_ids:  Option<Vec<i32>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct LanguageArg {
    pub(crate) lang_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for LanguageArg {
    fn default() -> Self {
        Self {
            lang_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptLanguageArg> for LanguageArg {
    fn from(data: IptLanguageArg) -> Self {
        let IptLanguageArg {
            lang_ids,
            limit,
            offset,
        } = data;

        Self {
            lang_ids: lang_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
