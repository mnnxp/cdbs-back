use crate::errors::ServiceResult;
use crate::models::company::company_represent::representation_type::model::RepresentationTypeTranslateList;
use diesel::PgConnection;

/// Возвращает список типов представительств (подразделений) компаний.
pub(crate) fn get_types_for_represent(
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<RepresentationTypeTranslateList>> {
    RepresentationTypeTranslateList::get_all(set_lang_id, conn)
}
