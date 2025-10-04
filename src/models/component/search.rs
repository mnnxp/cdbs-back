// Функции и запросы для полнотекстового поиска
// Поиск по:
// имени и описанию компонента;
// параметрам компонента;
// каталогам компонента;
// ключевым словам компонента.
// Результатом поиска является список из Uuid найденных объектов.

// use crate::models::search::model::ExtraOptions;
use crate::errors::ServiceResult;
use crate::models::search::{
    filter::{objects_search, Filter},
    model::IptSearchArg,
};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn search_components(
    args: &IptSearchArg,
    filter_uuids: Vec<Uuid>,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let filter = Filter::parsing("uuid", &filter_uuids);
    let mut res = search_in_components(args, &filter, conn)?;
    if args.by_params {
        res.append(&mut search_in_component_params(args, &filter, conn)?);
    }
    if args.by_specs {
        res.append(&mut search_in_component_specs(args, &filter, conn)?);
    }
    if args.by_keywords {
        res.append(&mut search_in_component_keywords(args, &filter, conn)?);
    }
    Ok(res)
}

/// Returns a list of Uuid of matching components
pub(crate) fn search_in_components(
    args: &IptSearchArg,
    filter: &Filter,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    objects_search(
        "component_ref",
        "make_tsvector(component_ref.name, component_ref.description)",
        &args.search,
        filter,
        conn,
    )
}

pub(crate) fn search_in_component_params(
    args: &IptSearchArg,
    filter: &Filter,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    objects_search(
        "component_ref LEFT JOIN param_to_component AS ptc ON ptc.component_uuid = component_ref.uuid LEFT JOIN param_ref AS pr ON pr.id = ptc.param_id",
        "to_tsvector(ptc.value)",
        &args.search,
        filter,
        conn)
}

pub(crate) fn search_in_component_specs(
    args: &IptSearchArg,
    filter: &Filter,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    objects_search(
        "component_ref LEFT JOIN spec_to_component AS stc ON stc.component_uuid = component_ref.uuid LEFT JOIN spec_translate_list AS stl ON stl.spec_id = stc.spec_id",
        "to_tsvector(stl.spec)",
        &args.search,
        filter,
        conn
    )
}

pub(crate) fn search_in_component_keywords(
    args: &IptSearchArg,
    filter: &Filter,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    objects_search(
        "component_ref LEFT JOIN keyword_to_component AS ktc ON ktc.component_uuid = component_ref.uuid LEFT JOIN keyword_ref AS kr ON kr.id = ktc.keyword_id",
        "to_tsvector(kr.keyword)",
        &args.search,
        filter,
        conn
    )
}
