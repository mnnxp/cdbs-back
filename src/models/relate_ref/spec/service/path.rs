use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::spec::model::{
    Spec, SpecTranslateList,
};
use diesel::PgConnection;

/// Collecting full path for specification
pub(crate) fn collect_path_spec(
    spec_id: &i32,
    split_char: &char,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<String> {
    let target_specs_ids = get_parents_ids(
        spec_id,
        conn
    ).map_err(|err| {
        debug!("Failed get parents ids: {}", err);
        ServiceError::BadRequest("Spec not found".to_string())
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

/// Get all parents specs up to root
fn get_parents_ids(
    spec_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<i32>> {
    let mut spec_levels: Vec<i32> = vec![*spec_id];
    let mut spec_id: i32 = *spec_id;

    loop {
        let spec: Spec = Spec::get_by_id(
            &spec_id,
            conn
        )?;

        if spec.id == spec.parent_spec_id ||
            spec_levels.len() > 50 {
            break;
        }

        spec_levels.push(spec.parent_spec_id);
        spec_id = spec.parent_spec_id;
    }

    Ok(spec_levels)
}

/// Parsing specs data and collect patch for target lang
fn get_path_from_specs(
    specs_data: &[SpecTranslateList],
    split_char: &char,
) -> String {
    let mut path_spec = String::new();
    let split = format!(" {} ", split_char);

    for (i, sd) in specs_data.iter().enumerate() {
        // no include splits before root level
        if i > 0 {
            path_spec += split.as_str();
        }

        path_spec += sd.spec.as_str();
    }

    path_spec
}
