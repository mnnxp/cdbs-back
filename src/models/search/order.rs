use super::model::ObjectUuid;
use crate::errors::{ServiceResult, ServiceError};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct Paginate {
    pub(crate) limit: i64,
    pub(crate) offset: i64,
}

impl Default for Paginate {
    /// Default arguments limit=1000 and offset=0
    fn default() -> Self {
        Self {
            limit: 1000,
            offset: 0,
        }
    }
}

impl Paginate {
    /// Sets the arguments for pagination.
    /// If the limit exceeds the offset by 1000, the default value is returned
    pub(crate) fn parsing(limit: i32, offset: i32) -> Self {
        if (limit - offset) > 1000 {
            debug!("Invalid limit {} for a specified offset {}.", limit, offset);
            return Self::default()
        }
        Self {
            limit: limit as i64,
            offset: offset as i64,
        }
    }

    /// Returns `LIMIT...OFFSET...` string with pagination parameters
    fn get_complete(&self) -> String {
        format!("LIMIT {} OFFSET {}", self.limit, self.offset)
    }
}

#[derive(Debug)]
pub(crate) struct TableColumn {
    table: String,
    column: String,
}

impl TableColumn {
    /// Returns the structure after table and column mapping (minimal validation).
    /// The column value can be set by default (if there is no match).
    fn parsing(table: &str, column: &str) -> Self {
        if table != "component_ref" {
            debug!("Fields for the {} table are not described.", table);
        }
        // if table == "component_ref" { ... }
        Self {
            table: table.to_string(),
            column: match column {
                "name" => "name".to_string(),
                "update" => "updated_at".to_string(),
                _ => "created_at".to_string(),
            }
        }
    }

    /// Returns string `FROM...` with text of table field
    fn get_from(&self) -> String {
        format!("FROM {}", self.table)
    }

    /// Returns string `table.column` with names of table and column
    fn get_with_point(&self) -> String {
        format!("{}.{}", self.table, self.column)
    }
}

#[derive(Debug)]
pub(crate) enum Sort {
    Asc(TableColumn),
    Desc(TableColumn),
}

impl Sort {
    pub(crate) fn set_by_table(table: &str) -> Self {
        Self::Asc(TableColumn::parsing(table, ""))
    }

    pub(crate) fn parsing(table: &str, order_by: &str, as_desc: bool) -> Sort {
        match as_desc {
            true => Sort::Desc(TableColumn::parsing(table, order_by)),
            false => Sort::Asc(TableColumn::parsing(table, order_by)),
        }
    }

    /// Returns string `FROM...` with text of table field
    fn get_from(&self) -> String {
        match self {
            Self::Desc(ob) => ob.get_from(),
            Self::Asc(ob) => ob.get_from(),
        }
    }

    /// Returns string `ORDER BY...` with sorting options, or an empty string if no arguments are found
    fn get_complete(&self) -> String {
        match self {
            Self::Desc(ob) => format!("ORDER BY {} DESC", ob.get_with_point()),
            Self::Asc(ob) => format!("ORDER BY {} Asc", ob.get_with_point()),
        }
    }
}

/// Returns a sorted list of ids or an error from the database
pub(crate) fn objects_order(
    object_uuids: &[Uuid],
    sort: &Sort,
    paginate: &Paginate,
    conn: &mut PgConnection
) -> ServiceResult<Vec<Uuid>> {
    let zero_point = Uuid::nil().to_string();
    let query = format!("
    SELECT uuid
    {from}
    WHERE uuid IN ('{object_uuids}')
    {sort}
    {paginate}",
        from = sort.get_from(),
        object_uuids = object_uuids.iter().fold(zero_point, |acc, &x| format!("{acc}', '{x}")),
        sort = sort.get_complete(),
        paginate = paginate.get_complete(),
    );
    debug!("SQL search query: {}", query);

    let temp: Vec<ObjectUuid> = diesel::sql_query(query)
        .load(conn)
        .map_err(|err| {
            debug!("Failed to sort uuid: {:?}", err);
            ServiceError::InternalServerError
        })?;
    Ok(ObjectUuid::get_uuids(&temp))
}