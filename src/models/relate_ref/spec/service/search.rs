use super::path::get_paths_specs;
use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::{SearchSpecArg, SpecId, SpecPath, SpecPathArg};
use crate::models::search::order::Paginate;
use diesel::PgConnection;

/// Возвращает пути к разделам каталога, найденным по наименованию.
/// При создании пути раздела используется заданный разделитель или разделитель по умолчанию "/".
/// Значение "deep_level" устанавливает предел глубины до родительского раздела.
pub(crate) fn search_specs_by_name(
    args: &SearchSpecArg,
    set_lang_id: i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<SpecPath>> {
    if args.text.is_empty() {
        return Ok(Vec::new());
    }
    let res_query = SpecId::get_list_by_name(&args.text, set_lang_id, conn)?;

    let mut target_specs_ids: Vec<i32> = Vec::new();
    for value in res_query {
        target_specs_ids.push(value.spec_id)
    }

    if target_specs_ids.is_empty() {
        return Ok(Vec::new());
    }

    get_paths_specs(
        &SpecPathArg {
            spec_ids: target_specs_ids,
            split_char: args.split_char,
            depth_level: args.depth_level,
        },
        set_lang_id,
        paginate,
        conn,
    )
}
