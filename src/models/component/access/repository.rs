use crate::auth::AccessOperation;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::search::get_vec_in_string;
use crate::models::search::model::ObjectUuid;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns UUIDs of components accessible to `user_uuid` with at least `operation` level.
///
/// Access is granted if any of the following holds:
/// - user is the owner (`component_ref.user_uuid = user_uuid`),
/// - component is public (`component_ref.type_access_id = 3`) — only for `Read`,
/// - user has direct access (`user_access_to_component`),
/// - user is a member of a company that has access
///   (`company_access_to_component` + `company_member_list` + `role_access`).
///
/// `filter_uuids` narrows the result to those UUIDs (empty = all).
pub(crate) fn get_accessible_component_uuids(
    user_uuid: &Uuid,
    operation: AccessOperation,
    filter_uuids: &[Uuid],
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let need_level = operation.to_access_level();

    let filter = if filter_uuids.is_empty() {
        String::new()
    } else {
        format!("AND c.uuid IN ({})", get_vec_in_string(filter_uuids))
    };

    // public (type_access_id = 3) grants Read only
    let public_clause = if need_level == 3 {
        "OR c.type_access_id = 3"
    } else {
        ""
    };

    let query = format!(
        r#"
        SELECT DISTINCT c.uuid
        FROM component_ref c
        WHERE c.is_delete = false
          {filter}
          AND (
              c.user_uuid = '{user}'
              {public_clause}
              OR EXISTS (
                  SELECT 1 FROM user_access_to_component ua
                  WHERE ua.component_uuid = c.uuid
                    AND ua.user_uuid = '{user}'
                    AND ua.type_access_id <= {need_level}
                    AND ua.is_enabled = true
              )
              OR EXISTS (
                  SELECT 1 FROM company_access_to_component ca
                  JOIN company_member_list cm
                    ON cm.company_uuid = ca.company_uuid
                  JOIN role_access ra
                    ON ra.role_id = cm.role_id
                  WHERE ca.component_uuid = c.uuid
                    AND cm.user_uuid = '{user}'
                    AND cm.is_enabled = true
                    AND ca.is_enabled = true
                    AND ra.type_access_id <= {need_level}
              )
          )
        "#,
        filter = filter,
        user = user_uuid,
        public_clause = public_clause,
        need_level = need_level,
    );
    debug!("SQL accessible components query: {}", query);

    diesel::sql_query(query)
        .load::<ObjectUuid>(conn)
        .map(|v| ObjectUuid::get_uuids(&v))
        .map_err(|e| {
            debug!("Failed get accessible components: {:?}", e);
            ServiceError::InternalServerError
        })
}
