use crate::auth::AccessOperation;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::search::get_vec_in_string;
use crate::models::search::model::ObjectUuid;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns UUIDs of standards accessible to `user_uuid` with at least `operation` level.
///
/// Access is granted if any of the following holds:
/// - user is the owner (`standard_ref.user_uuid = user_uuid`),
/// - standard is public (`standard_ref.type_access_id = 3`) — only for `Read`,
/// - user has direct access (`user_access_to_standard`),
/// - user is a member of a company that has access
///   (`company_access_to_standard` + `company_member_list` + `role_access`).
///
/// `filter_uuids` narrows the result to those UUIDs (empty = all).
pub(crate) fn get_accessible_standard_uuids(
    user_uuid: &Uuid,
    operation: AccessOperation,
    filter_uuids: &[Uuid],
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let need_level = operation.to_access_level();

    let filter = if filter_uuids.is_empty() {
        String::new()
    } else {
        format!("AND s.uuid IN ({})", get_vec_in_string(filter_uuids))
    };

    let public_clause = if need_level == 3 {
        "OR s.type_access_id = 3"
    } else {
        ""
    };

    let query = format!(
        r#"
        SELECT DISTINCT s.uuid
        FROM standard_ref s
        WHERE s.is_delete = false
          {filter}
          AND (
              s.user_uuid = '{user}'
              {public_clause}
              OR EXISTS (
                  SELECT 1 FROM user_access_to_standard ua
                  WHERE ua.standard_uuid = s.uuid
                    AND ua.user_uuid = '{user}'
                    AND ua.type_access_id <= {need_level}
              )
              OR EXISTS (
                  SELECT 1 FROM company_access_to_standard ca
                  JOIN company_member_list cm
                    ON cm.company_uuid = ca.company_uuid
                  JOIN role_access ra
                    ON ra.role_id = cm.role_id
                  WHERE ca.standard_uuid = s.uuid
                    AND cm.user_uuid = '{user}'
                    AND cm.is_enabled = true
                    AND ra.type_access_id <= {need_level}
              )
          )
        "#,
        filter = filter,
        user = user_uuid,
        public_clause = public_clause,
        need_level = need_level,
    );
    debug!("SQL accessible standards query: {}", query);

    diesel::sql_query(query)
        .load::<ObjectUuid>(conn)
        .map(|v| ObjectUuid::get_uuids(&v))
        .map_err(|e| {
            debug!("Failed get accessible standards: {:?}", e);
            ServiceError::InternalServerError
        })
}
