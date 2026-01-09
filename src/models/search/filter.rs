use super::{get_vec_in_string, model::ObjectUuid};
use crate::errors::{ServiceError, ServiceResult};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) struct Filter {
    field_name: String,
    filter_items: Vec<Uuid>,
}

impl Filter {
    pub(crate) fn parsing(field_name: &str, object_uuids: &[Uuid]) -> Filter {
        if field_name.is_empty() || object_uuids.is_empty() {
            return Filter {
                field_name: "".to_string(),
                filter_items: Vec::new(),
            };
        }
        Filter {
            field_name: field_name.to_string(),
            filter_items: object_uuids.to_vec(),
        }
    }

    /// Returns string `AND..` with additional filtering options
    pub(crate) fn get_complete(&self) -> String {
        if self.field_name.is_empty() || self.filter_items.is_empty() {
            return String::new();
        }
        if self.filter_items.len() == 1 {
            if let Some(filter_item) = self.filter_items.first() {
                return format!("AND {} = \'{:?}\'", self.field_name, filter_item);
            }
        }
        format!(
            "AND {} IN ({})",
            self.field_name,
            get_vec_in_string(&self.filter_items)
        )
    }
}

/// Creates and sends a full-text search request,
/// returns ids of found objects or an error from the database
pub(crate) fn objects_search(
    from: &str,
    to_tsvector: &str,
    search: &str,
    filter: &Filter,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let query = format!("
    SELECT uuid
    FROM {from}
    WHERE {to_tsvector} @@ websearch_to_tsquery('{search}')
    {filter}
    LIMIT 1000;",
        from = from,
        to_tsvector = to_tsvector,
        search = search,
        filter = filter.get_complete(),
    );
    debug!("SQL search query: {}", query);

    let temp: Vec<ObjectUuid> = diesel::sql_query(query).load(conn).map_err(|err| {
        debug!("Failed search: {:?}", err);
        ServiceError::InternalServerError
    })?;
    Ok(ObjectUuid::get_uuids(&temp))
}
