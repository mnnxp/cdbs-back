// use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::language::model::SetLangName;
// use crate::schema::language_ref::dsl as language_ref;
// use diesel::prelude::*;

impl SetLangName {
    /// Gets English name language by lang_id
    pub(crate) fn get_by_id(target_language_id: i32) -> SetLangName {
        match target_language_id {
            2 => "russian".into(),
            _ => "english".into(),
        }
    }
}
