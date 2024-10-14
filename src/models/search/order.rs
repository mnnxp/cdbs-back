use diesel::prelude::*;
use uuid::Uuid;
use crate::errors::{ServiceResult, ServiceError};
use super::model::ObjectUuid;

#[derive(Debug)]
pub(crate) struct Paginate {
    pub(crate) limit: i64,
    pub(crate) offset: i64,
}

impl Default for Paginate {
    /// Default arguments limit=100 and offset=0
    fn default() -> Self {
        Self {
            limit: 100,
            offset: 0,
        }
    }
}

impl Paginate {
    /// Sets the limit and offset for pagination
    /// by specifying current page and number of elements per page.
    pub(crate) fn parsing_by_page(current_page: i32, per_page: i32) -> Self {
        if current_page <= 0 || per_page <= 0 {
            debug!("Invalid current page {} or a specified per page {}.", current_page, per_page);
            return Self::default()
        }
        Self {
            limit: per_page as i64,
            offset: ((current_page - 1) * per_page) as i64,
        }
    }

    /// Sets the arguments for pagination.
    /// If the limit exceeds the offset by 500, the default value is returned
    pub(crate) fn parsing(limit: i32, offset: i32) -> Self {
        if (limit - offset) > 500 {
            debug!("Invalid limit {} for a specified offset {}.", limit, offset);
            return Self::default()
        }
        Self {
            limit: limit as i64,
            offset: offset as i64,
        }
    }

    /// Returns `LIMIT...OFFSET...` string with pagination parameters
    pub(crate) fn get_complete(&self) -> String {
        format!("LIMIT {} OFFSET {}", self.limit, self.offset)
    }
}

#[derive(Debug)]
pub(crate) enum TableName {
    ComponentRef,
    ComponentModification,
    FileRef,
    ParamTranslateList,
}

impl TableName {
    // Returns table name, e.g. `name_ref`
    fn name(&self) -> &str {
        match self {
            Self::ComponentRef => "component_ref",
            Self::ComponentModification => "component_modification_list",
            Self::FileRef => "file_ref",
            Self::ParamTranslateList => "", // table is specified in fields
        }
    }
}

#[derive(Debug)]
pub(crate) struct TableColumn {
    table: TableName,
    column: String,
}

impl TableColumn {
    /// Returns the structure after table and column mapping (minimal validation).
    /// The column value can be set to default or empty if there are no table matches.
    fn parsing(table: TableName, field: &str) -> Self {
        match table {
            TableName::ComponentRef => Self {
                table,
                column: match field {
                    "name" => "name".to_string(),
                    "actualStatusId" => "actual_status_id".to_string(),
                    "updatedAt" => "updated_at".to_string(),
                    _ => "created_at".to_string(),
                }
            },
            TableName::ComponentModification => Self {
                table,
                column: match field {
                    "name" => "modification_name".to_string(),
                    "actualStatusId" => "actual_status_id".to_string(),
                    "updatedAt" => "updated_at".to_string(),
                    _ => "created_at".to_string(),
                }
            },
            TableName::FileRef => Self {
                table,
                column: match field {
                    "revision" => "revision".to_string(),
                    "filename" => "filename".to_string(),
                    "size" => "filesize".to_string(),
                    "updatedAt" => "updated_at".to_string(),
                    _ => "created_at".to_string(),
                }
            },
            TableName::ParamTranslateList => Self {
                table,
                column: match field {
                    "value" => "pt.value".to_string(),
                    "paramname" => "ptl.paramname".to_string(),
                    _ => "ptl.param_id".to_string(),
                }
            },
        }
    }

    /// Returns string `FROM...` with text of table field
    fn get_from(&self) -> String {
        format!("FROM {}", self.table.name())
    }

    /// Returns string `table.column` with names of table and column
    fn get_with_point(&self) -> String {
        if self.table.name().is_empty() {
            // if a table is specified in fields (small hack)
            return self.column.clone()
        }
        format!("{}.{}", self.table.name(), self.column)
    }

    /// Returns false if column name is empty
    fn is_empty(&self) -> bool {
        // self.table.is_empty() && self.column.is_empty()
        self.column.is_empty()
    }
}

#[derive(Debug)]
pub(crate) enum Sort {
    Asc(TableColumn),
    Desc(TableColumn),
}

impl Sort {
    pub(crate) fn set_by_table(table: TableName) -> Self {
        Self::Asc(TableColumn::parsing(table, ""))
    }

    pub(crate) fn parsing(table: TableName, field: &str, as_desc: bool) -> Sort {
        match as_desc {
            true => Sort::Desc(TableColumn::parsing(table, field)),
            false => Sort::Asc(TableColumn::parsing(table, field)),
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
    pub(crate) fn get_complete(&self) -> String {
        match self {
            Self::Desc(ob) if !ob.is_empty() => format!("ORDER BY {} DESC", ob.get_with_point()),
            Self::Asc(ob) if !ob.is_empty() => format!("ORDER BY {} ASC", ob.get_with_point()),
            _ => String::new(),
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
    debug!("SQL objects order query: {}", query);

    let temp: Vec<ObjectUuid> = diesel::sql_query(query)
        .load(conn)
        .map_err(|err| {
            debug!("Failed to sort uuid: {:?}", err);
            ServiceError::InternalServerError
        })?;
    Ok(ObjectUuid::get_uuids(&temp))
}