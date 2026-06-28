// Full-text search functions and queries
// Search by:
// - component name and description
// - component parameters
// - component catalogs (specs)
// - component keywords
// - component modifications
// - modification parameters
// Result is a list of found object UUIDs.

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
    if args.by_modifications {
        res.append(&mut search_in_modifications(args, &filter, conn)?);
    }
    if args.by_modification_params {
        res.append(&mut search_in_modification_params(args, &filter, conn)?);
    }

    Ok(res)
}

/// Search in component name and description
pub(crate) fn search_in_components(
    args: &IptSearchArg,
    filter: &Filter,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    objects_search(
        "component_ref",
        "component_ref.uuid",
        "make_tsvector(component_ref.name, component_ref.description)",
        &args.search,
        filter,
        conn,
    )
}

/// Search in component parameters (param_to_component.value)
pub(crate) fn search_in_component_params(
    args: &IptSearchArg,
    filter: &Filter,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    objects_search(
        "component_ref LEFT JOIN param_to_component AS ptc ON ptc.component_uuid = component_ref.uuid",
        "component_ref.uuid",
        "to_tsvector(ptc.value)",
        &args.search,
        filter,
        conn,
    )
}

/// Search in component specs (spec_translate_list.spec)
pub(crate) fn search_in_component_specs(
    args: &IptSearchArg,
    filter: &Filter,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    objects_search(
        "component_ref LEFT JOIN spec_to_component AS stc ON stc.component_uuid = component_ref.uuid LEFT JOIN spec_translate_list AS stl ON stl.spec_id = stc.spec_id",
        "component_ref.uuid",
        "to_tsvector(stl.spec)",
        &args.search,
        filter,
        conn,
    )
}

/// Search in component keywords (keyword_ref.keyword)
pub(crate) fn search_in_component_keywords(
    args: &IptSearchArg,
    filter: &Filter,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    objects_search(
        "component_ref LEFT JOIN keyword_to_component AS ktc ON ktc.component_uuid = component_ref.uuid LEFT JOIN keyword_ref AS kr ON kr.id = ktc.keyword_id",
        "component_ref.uuid",
        "to_tsvector(kr.keyword)",
        &args.search,
        filter,
        conn,
    )
}

/// Search in component modifications (modification_name and description)
pub(crate) fn search_in_modifications(
    args: &IptSearchArg,
    filter: &Filter,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    objects_search(
        "component_ref LEFT JOIN component_modification_list AS cml ON cml.component_uuid = component_ref.uuid",
        "component_ref.uuid",
        "make_tsvector(cml.modification_name, COALESCE(cml.description, ''))",
        &args.search,
        filter,
        conn,
    )
}

/// Search in component modification parameters (param_to_modification.value)
pub(crate) fn search_in_modification_params(
    args: &IptSearchArg,
    filter: &Filter,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    objects_search(
        "component_ref LEFT JOIN component_modification_list AS cml ON cml.component_uuid = component_ref.uuid LEFT JOIN param_to_modification AS ptm ON ptm.modification_uuid = cml.uuid",
        "component_ref.uuid",
        "to_tsvector(ptm.value)",
        &args.search,
        filter,
        conn,
    )
}