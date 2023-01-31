// use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::language::model::EngLangName;
// use crate::schema::language_ref::dsl as language_ref;
// use diesel::prelude::*;

impl EngLangName {
    /// Gets English name language by lang_id
    pub(crate) fn get_by_id(
        target_language_id: &i32,
        // conn: &mut PgConnection,
    ) -> EngLangName {
        // language_ref::language_ref
        //     .filter(language_ref::id.eq(target_language_id))
        //     .select(language_ref::eng_lang_name)
        //     .first::<EngLangName>(conn)
        //     .map_err(|err| {
        //         debug!("Failed get lang name: {:?}", err);
        //         ServiceError::InternalServerError
        //     })

        match *target_language_id {
            2 => "russian".into(),
            _ => "english".into()
        }
    }
}
