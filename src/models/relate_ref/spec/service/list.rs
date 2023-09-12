use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::{
    SpecTranslateList, SpecArg
};
use diesel::PgConnection;

/// Возвращает разделы каталога.
/// Можно указать раздел верхнего уровня (родительский), от которого будет формироваться список.
/// Независимо от указания верхнего раздела, доступно задание фильтра по идентификаторам разделов.
pub(crate) fn get_specs(
    arguments: &SpecArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    let SpecArg {
        spec_ids,
        specs_levels,
        limit,
        offset,
    } = arguments;

    match specs_levels.is_empty() {
        true => {
            SpecTranslateList::get_by_ids(
                spec_ids,
                limit,
                offset,
                set_lang_id,
                conn
            )
        },
        false => {
            SpecTranslateList::get_by_parent_ids(
                spec_ids,
                specs_levels,
                limit,
                offset,
                set_lang_id,
                conn
            )
        },
    }
}
