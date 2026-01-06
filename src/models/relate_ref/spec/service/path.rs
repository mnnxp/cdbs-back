use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::spec::model::{Spec, SpecPath, SpecPathArg, SpecTranslateList};
use crate::models::search::order::Paginate;
use diesel::{prelude::*, PgConnection};

/// Returns paths to catalogs by ID.
/// When creating a catalog path, the specified separator or default separator "/" is used.
/// A value of `deep_level` sets the depth limit to the parent catalog.
pub(crate) fn get_paths_specs(
    args: &SpecPathArg,
    set_lang_id: &i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<SpecPath>> {
    let select_ids = get_spec_ids(&args.spec_ids, paginate, conn)?;
    if select_ids.len() > 100 {
        return Err(get_err_msg(ErrorMessage::NotMorePathInOneQuery));
    }
    let mut result: Vec<SpecPath> = Vec::new();
    for sid in &select_ids {
        result.push(SpecPath {
            spec_id: *sid,
            lang_id: *set_lang_id,
            path: collect_path_spec(sid, &args.split_char, &args.depth_level, set_lang_id, conn)?,
        });
    }
    Ok(result)
}

/// Gets spec ids from db without filter
fn get_spec_ids(
    spec_ids: &[i32],
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<i32>> {
    use crate::schema::spec_ref::dsl as spec_ref;
    let mut query = spec_ref::spec_ref.into_boxed();
    if !spec_ids.is_empty() {
        query = query.filter(spec_ref::id.eq_any(spec_ids));
    }
    let res_ids = query
        .select(spec_ref::id)
        .offset(paginate.offset)
        .limit(paginate.limit)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed get spec ids: {}", err);
            ServiceError::InternalServerError
        })?;
    if !spec_ids.is_empty() && res_ids.is_empty() {
        return Err(get_err_msg(ErrorMessage::SpecNotFound));
    }
    Ok(res_ids)
}

/// Collecting full path
fn collect_path_spec(
    spec_id: &i32,
    split_char: &char,
    depth_level: &i32,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<String> {
    let target_specs_ids = get_parents_ids(*spec_id, *depth_level, conn).map_err(|err| {
        debug!("Failed get parents ids: {}", err);
        ServiceError::InternalServerError
    })?;

    let target_specs_data =
        SpecTranslateList::get_by_ids(&target_specs_ids, set_lang_id, &Paginate::default(), conn)
            .map_err(|err| {
            debug!("Failed get spec data by ids: {}", err);
            ServiceError::InternalServerError
        })?;

    Ok(get_path_from_specs(&target_specs_data, split_char))
}

/// Get all parents specs up to setting depth level
fn get_parents_ids(
    spec_id: i32,
    depth_level: i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<i32>> {
    let spec = Spec::get_by_id(&spec_id, conn)?;
    let mut specs_levels = Vec::new();
    let mut count = 0;
    for part in spec.path.split('.').rev() {
        if let Ok(num) = part.parse::<i32>() {
            specs_levels.push(num);
            count += 1;
            if depth_level > 0 && count >= depth_level {
                break;
            }
        }
    }
    Ok(specs_levels)
}

/// Parsing specs data and collect patch for target lang
fn get_path_from_specs(specs_data: &[SpecTranslateList], split_char: &char) -> String {
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
