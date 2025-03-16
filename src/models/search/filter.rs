use diesel::prelude::*;
use uuid::Uuid;
use crate::errors::{ServiceResult, ServiceError};
use super::{get_vec_in_string, model::ObjectUuid};

pub(crate) struct Filter {
    field_name: String,
    filter_items: String,
}

impl Filter {
    pub(crate) fn parsing(field_name: &str, object_uuids: &[Uuid]) -> Filter {
        if field_name.is_empty() || object_uuids.is_empty() {
            return Filter {
                field_name: "".to_string(),
                filter_items: "".to_string(),
            }
        }
        Filter {
            field_name: field_name.to_string(),
            filter_items: get_vec_in_string(object_uuids)
        }
    }

    /// Returns string `AND..` with additional filtering options
    fn get_complete(&self) -> String {
        if self.field_name.is_empty() || self.filter_items.is_empty() {
            return String::new()
        }
        format!("AND {} IN ({})", self.field_name, self.filter_items)
    }
}

/// Creates and sends a full-text search request,
/// returns ids of found objects or an error from the database
pub(crate) fn objects_search(
    from: &str,
    to_tsvector: &str,
    search: &str,
    filter: &Filter,
    conn: &mut PgConnection
) -> ServiceResult<Vec<Uuid>> {
    let query = format!("
    SELECT uuid
    FROM {from}
    WHERE {to_tsvector} @@ websearch_to_tsquery('{search}')
    {filter}
    LIMIT 1000",
        from = from,
        to_tsvector = to_tsvector,
        search = search,
        filter = filter.get_complete(),
    );
    debug!("SQL search query: {}", query);

    let temp: Vec<ObjectUuid> = diesel::sql_query(query)
        .load(conn)
        .map_err(|err| {
            debug!("Failed search uuid: {:?}", err);
            ServiceError::InternalServerError
        })?;
    Ok(ObjectUuid::get_uuids(&temp))
}