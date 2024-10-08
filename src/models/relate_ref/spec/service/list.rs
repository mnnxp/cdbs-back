use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::{
    SpecTranslateList, SpecArg
};
use crate::models::search::order::Paginate;
use diesel::PgConnection;

/// Возвращает разделы каталога.
/// Можно указать раздел верхнего уровня (родительский), от которого будет формироваться список.
/// Независимо от указания верхнего раздела, доступно задание фильтра по идентификаторам разделов.
pub(crate) fn get_specs(
    args: &SpecArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    match args.specs_levels.is_empty() {
        true => {
            SpecTranslateList::get_by_ids(
                &args.spec_ids,
                set_lang_id,
                &Paginate::parsing(args.limit, args.offset),
                conn
            )
        },
        false => {
            SpecTranslateList::get_by_parent_ids(
                &args.spec_ids,
                &args.specs_levels,
                set_lang_id,
                &Paginate::parsing(args.limit, args.offset),
                conn
            )
        },
    }
}
