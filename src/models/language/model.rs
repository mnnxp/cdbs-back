use crate::schema::*;
use async_graphql::*;
// use chrono::*;

// Language models

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Language {
    pub id: i32,
    pub lang: String,
    pub langshort: String,
}

#[Object]
impl Language {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn lang(&self) -> &String {
        &self.lang
    }
    async fn langshort(&self) -> &String {
        &self.langshort
    }
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

// for request
pub struct SetLang {
    pub id_lang: i32,
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
