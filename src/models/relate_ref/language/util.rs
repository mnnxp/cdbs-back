use crate::models::relate_ref::language::model::SetLang;

use async_graphql::Context;

/// get the id of the language for the user interface
/// (if not specified in the request, it will be 1)
pub(crate) fn get_set_language(
    cxt: &Context<'_>,
) -> i32 {
    match cxt.data_opt::<SetLang>() {
        Some(set_lang) => set_lang.lang_id,
        None => 1, // <-- default language
    }
}
