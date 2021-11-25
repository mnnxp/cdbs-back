use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::spec::model::{
    Spec, SpecTranslateList, SpecPath, SpecPathArg
};
use diesel::{PgConnection, prelude::*};

/// Gets full paths for specifications
pub(crate) fn get_paths_specs(
    arguments: &SpecPathArg,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<SpecPath>> {
    let SpecPathArg {
        spec_ids,
        split_char,
        depth_level,
        limit,
        offset,
    } = arguments;

    let select_ids = get_spec_ids(spec_ids, limit, offset, conn)?;
    if select_ids.len() > 100 {
        return Err(ServiceError::BadRequest("Not more 100 path in one query".to_string()));
    }

    let mut result: Vec<SpecPath> = Vec::new();
    for sid in &select_ids {
        result.push(SpecPath{
            spec_id: *sid,
            lang_id: *set_lang_id,
            path: collect_path_spec(
                sid,
                split_char,
                depth_level,
                set_lang_id,
                conn
            )?
        });
    }

    Ok(result)
}

/// Gets spec ids from db without filter
fn get_spec_ids(
    spec_ids: &[i32],
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<i32>> {
    use crate::schema::spec_ref::dsl as spec_ref;

    let mut query = spec_ref::spec_ref.into_boxed();
    if !spec_ids.is_empty() {
        query = query.filter(spec_ref::id.eq_any(spec_ids));
    }

    let res_ids = query.select(spec_ref::id)
        .offset(*offset as i64)
        .limit(*limit as i64)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed get spec ids: {}", err);
            ServiceError::InternalServerError
        })?;

    if !spec_ids.is_empty() && res_ids.is_empty() {
        return Err(ServiceError::BadRequest("Spec not found".to_string()));
    }

    Ok(res_ids)
}

/// Collecting full path for specification
fn collect_path_spec(
    spec_id: &i32,
    split_char: &char,
    depth_level: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<String> {
    let target_specs_ids = get_parents_ids(
        spec_id,
        depth_level,
        conn
    ).map_err(|err| {
        debug!("Failed get parents ids: {}", err);
        ServiceError::InternalServerError
    })?;

    let target_specs_data = SpecTranslateList::get_by_ids(
        &target_specs_ids,
        &100,
        &0,
        set_lang_id,
        conn
    ).map_err(|err| {
        debug!("Failed get spec data by ids: {}", err);
        ServiceError::InternalServerError
    })?;

    Ok(get_path_from_specs(
        &target_specs_data,
        split_char
    ))
}

/// Get all parents specs up to setting depth level
fn get_parents_ids(
    spec_id: &i32,
    depth_level: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<i32>> {
    let mut specs_levels: Vec<i32> = vec![*spec_id];
    let mut spec_id: i32 = *spec_id;

    let depth_level = match depth_level {
        50.. => 50_usize,
        _ => *depth_level as usize,
    };

    loop {
        let spec: Spec = Spec::get_by_id(
            &spec_id,
            conn
        )?;

        if spec.id == spec.parent_spec_id ||
            specs_levels.len() >= depth_level {
            break;
        }

        specs_levels.push(spec.parent_spec_id);
        spec_id = spec.parent_spec_id;
    }

    Ok(specs_levels)
}

/// Parsing specs data and collect patch for target lang
fn get_path_from_specs(
    specs_data: &[SpecTranslateList],
    split_char: &char,
) -> String {
    let mut path_spec = String::new();
    // let split = format!(" {} ", split_char);
    let split = split_char.to_string();
    let split_str = split.as_str();

    for (i, sd) in specs_data.iter().enumerate() {
        // no include splits before root level
        if i > 0 {
            path_spec += split_str;
        }

        path_spec += sd.spec.as_str();
    }

    path_spec
}
