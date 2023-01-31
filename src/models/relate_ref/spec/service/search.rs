use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::{
    SpecPath, SpecId, SearchSpecArg, SpecPathArg
};
use super::path::get_paths_specs;
use diesel::PgConnection;

/// Search specs by name among all language
pub(crate) fn search_specs_by_name(
    arguments: &SearchSpecArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<SpecPath>> {
    let SearchSpecArg {
        text,
        split_char,
        depth_level,
        limit,
        offset,
    } = arguments;

    if text.is_empty() {
        return Ok(Vec::new());
    }

    let res_query = SpecId::get_list_by_name(
        text,
        limit,
        offset,
        set_lang_id,
        conn
    )?;

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
            split_char: *split_char,
            depth_level: *depth_level,
            limit: *limit,
            offset: *offset,
        },
        set_lang_id,
        conn
    )
}


// /// Search specs by name among all language
// pub(crate) fn search_specs_by_name(
//     text: &str,
//     // filter_specs_levels: &[i32],
//     limit: &i32,
//     offset: &i32,
//     set_lang_id: &i32,
//     conn: &mut PgConnection,
// ) -> ServiceResult<Vec<SpecTranslateList>> {
//     let res_query = SpecId::get_list_by_name(
//         text,
//         limit,
//         offset,
//         set_lang_id,
//         conn
//     )?;
//
//     let mut target_specs_ids: Vec<i32> = Vec::new();
//     for value in res_query {
//         target_specs_ids.push(value.spec_id)
//     }
//
//     if target_specs_ids.is_empty() {
//         return Ok(Vec::new());
//     }
//
//     SpecTranslateList::get_by_ids(
//         &target_specs_ids,
//         limit,
//         offset,
//         set_lang_id,
//         conn
//     )
// }
