use super::get_vec_in_string;
use super::model::{ObjectI64, ObjectUuid};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::discussion::model::DiscussionTo;
use diesel::prelude::*;
use uuid::Uuid;

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
    /// Sets the limit and offset for pagination by specifying current page
    /// and number of elements per page. Maximum per page: 1000.
    pub(crate) fn parsing_by_page(current_page: i32, per_page: i32) -> Self {
        if current_page <= 0 || per_page <= 0 {
            debug!(
                "Invalid current page {} or a specified per page {}.",
                current_page, per_page
            );
            return Self::default();
        }
        if per_page > 1000 {
            debug!("Invalid limit {}. Max: 1000.", per_page);
            return Self::default();
        }
        Self {
            limit: per_page as i64,
            offset: ((current_page - 1) * per_page) as i64,
        }
    }

    /// Returns `LIMIT...OFFSET...` string with pagination parameters
    pub(crate) fn get_complete(&self) -> String {
        format!("LIMIT {} OFFSET {}", self.limit, self.offset)
    }

    /// Returns total number of elements on which pagination is applied
    pub(crate) fn get_count(
        object_uuid: &Uuid,
        table_name: &TableName,
        conn: &mut PgConnection,
    ) -> ServiceResult<i64> {
        let column = table_name.relationship();
        if column.is_empty() {
            debug!("SQL query execution is impossible without a column name");
            return Err(ServiceError::InternalServerError);
        }
        // define the request to count object files
        let number_of_files = matches!(
            table_name,
            TableName::FileToComponent
                | TableName::FileToModification
                | TableName::FileToFilesetForProgram
        );
        // deleted and hidden files are not included in the calculation
        let query = match number_of_files {
            true => format!(
                "SELECT count(*) FROM {} INNER JOIN file_ref AS fr ON fr.uuid = file_uuid
                WHERE {} = '{}' AND fr.is_hidden = 'f' AND fr.is_delete = 'f'",
                table_name.name(),
                column,
                object_uuid
            ),
            false => format!(
                "SELECT count(*) FROM {} WHERE {} = '{}'",
                table_name.name(),
                column,
                object_uuid
            ),
        };
        debug!("SQL objects count query: {}", query);
        diesel::sql_query(query)
            .get_result::<ObjectI64>(conn)
            .map_err(|err| {
                debug!("Failed count number: {:?}", err);
                ServiceError::InternalServerError
            })
            .map(|res| res.count)
    }
}

#[derive(Debug)]
pub(crate) enum TableName {
    ComponentRef,
    ComponentModification,
    FileRef,
    ServiceRef,
    ParamTranslateList,
    ParamToComponent,
    ParamToService,
    FileToComponent,
    FileToService,
    StandardToComponent,
    SupplierToComponent,
    ParamToModification,
    FileToModification,
    FilesetForProgram,
    FileToFilesetForProgram,
    DiscussionRef(Option<DiscussionTo>),
    DiscussionCommentList,
}

impl TableName {
    /// Returns table name, e.g. `name_ref`
    fn name(&self) -> &str {
        match self {
            Self::ComponentRef => "component_ref",
            Self::ComponentModification => "component_modification_list",
            Self::FileRef => "file_ref",
            Self::ServiceRef => "service_ref",
            Self::ParamTranslateList => "", // table is specified in fields
            // Returns a name of a table for the relationship between two objects
            Self::ParamToComponent => "param_to_component",
            Self::ParamToService => "param_to_service",
            Self::FileToComponent => "file_to_component",
            Self::FileToService => "file_to_service",
            Self::StandardToComponent => "standard_to_component",
            Self::SupplierToComponent => "supplier_to_component",
            Self::ParamToModification => "param_to_modification",
            Self::FileToModification => "file_to_modification",
            Self::FilesetForProgram => "fileset_for_program",
            Self::FileToFilesetForProgram => "modification_file_from_fileset",
            Self::DiscussionRef(_) => "discussion_ref",
            Self::DiscussionCommentList => "discussion_comment_list",
        }
    }

    /// Returns column of table for the relationship between two objects
    fn relationship(&self) -> &str {
        match &self {
            Self::ComponentModification => "component_uuid",
            Self::ParamToComponent => "component_uuid",
            Self::ParamToService => "service_uuid",
            Self::FileToComponent => "component_uuid",
            Self::FileToService => "service_uuid",
            Self::StandardToComponent => "component_uuid",
            Self::SupplierToComponent => "component_uuid",
            Self::ParamToModification => "modification_uuid",
            Self::FileToModification => "modification_uuid",
            Self::FilesetForProgram => "modification_uuid",
            Self::FileToFilesetForProgram => "fileset_uuid",
            Self::DiscussionRef(discussion_to) => discussion_to
                .as_ref()
                .map(|dt| dt.get_relationship())
                .unwrap_or(""),
            Self::DiscussionCommentList => "discussion_uuid",
            _ => "",
        }
    }
}

impl DiscussionTo {
    /// Returns a table column with an identifier for the object associated with the discussion
    fn get_relationship(&self) -> &str {
        match self {
            Self::Company(_) => "company_uuid",
            Self::Component(_) => "component_uuid",
            Self::Service(_) => "service_uuid",
        }
    }
}

#[derive(Debug)]
enum DataType {
    String,
    Date,
    Number,
    None,
}

#[derive(Debug)]
pub(crate) struct TableColumn {
    table: TableName,
    column: String,
    data_type: DataType,
}

impl TableColumn {
    /// Returns the structure after table and column mapping (minimal validation).
    /// The column value can be set to default or empty if there are no table matches.
    fn parsing(table: TableName, field: &str) -> Self {
        match table {
            TableName::ComponentRef => {
                let (column, data_type) = match field {
                    "name" => ("name".to_string(), DataType::String),
                    "actualStatusId" => ("actual_status_id".to_string(), DataType::Number),
                    "updatedAt" => ("updated_at".to_string(), DataType::Date),
                    _ => ("created_at".to_string(), DataType::Date),
                };
                Self {
                    table,
                    column,
                    data_type,
                }
            }
            TableName::ComponentModification => {
                let (column, data_type) = match field {
                    "name" => ("modification_name".to_string(), DataType::String),
                    "actualStatusId" => ("actual_status_id".to_string(), DataType::Number),
                    "updatedAt" => ("updated_at".to_string(), DataType::Date),
                    _ => ("created_at".to_string(), DataType::Date),
                };
                Self {
                    table,
                    column,
                    data_type,
                }
            }
            TableName::FileRef => {
                let (column, data_type) = match field {
                    "revision" => ("revision".to_string(), DataType::Number),
                    "filename" => ("filename".to_string(), DataType::String),
                    "size" => ("filesize".to_string(), DataType::Number),
                    "updatedAt" => ("updated_at".to_string(), DataType::Date),
                    _ => ("created_at".to_string(), DataType::Date),
                };
                Self {
                    table,
                    column,
                    data_type,
                }
            }
            TableName::ServiceRef => {
                let (column, data_type) = match field {
                    "name" => ("name".to_string(), DataType::Number),
                    "description" => ("description".to_string(), DataType::String),
                    "serviceStatusId" => ("service_status_id".to_string(), DataType::Number),
                    "updatedAt" => ("updated_at".to_string(), DataType::Date),
                    _ => ("created_at".to_string(), DataType::Date),
                };
                Self {
                    table,
                    column,
                    data_type,
                }
            }
            TableName::ParamTranslateList => {
                let (column, data_type) = match field {
                    "value" => ("pt.value".to_string(), DataType::String),
                    "paramname" => ("ptl.paramname".to_string(), DataType::String),
                    _ => ("ptl.param_id".to_string(), DataType::Number),
                };
                Self {
                    table,
                    column,
                    data_type,
                }
            }
            TableName::DiscussionRef(_) => {
                let (column, data_type) = match field {
                    "title" => ("title".to_string(), DataType::String),
                    "lastActivityAt" => ("last_activity_at".to_string(), DataType::Date),
                    _ => ("created_at".to_string(), DataType::Date),
                };
                Self {
                    table,
                    column,
                    data_type,
                }
            }
            TableName::DiscussionCommentList => {
                let (column, data_type) = match field {
                    "parentComment" => ("parent_comment_uuid".to_string(), DataType::String),
                    "author" => ("author_uuid".to_string(), DataType::String),
                    "updatedAt" => ("updated_at".to_string(), DataType::Date),
                    _ => ("created_at".to_string(), DataType::Date),
                };
                Self {
                    table,
                    column,
                    data_type,
                }
            }
            _ => Self {
                table,
                column: String::new(),
                data_type: DataType::None,
            },
        }
    }

    /// Returns string `FROM...` with text of table field
    fn get_from(&self) -> String {
        format!("FROM {}", self.table.name())
    }

    /// Returns string `table.column` with names of table and column.
    /// For string-type columns, length is added to specify the sort order.
    fn get_with_point(&self) -> String {
        if self.table.name().is_empty() {
            // if a table is specified in fields (small hack)
            return self.column.clone();
        }
        let point = format!("{}.{}", self.table.name(), self.column);
        match self.data_type {
            DataType::String => format!("(length({}), {})", point, point),
            _ => point,
        }
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
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    if object_uuids.is_empty() {
        return Ok(Vec::new());
    }
    let query = format!(
        "
    SELECT uuid
    {from}
    WHERE uuid IN ({object_uuids})
    {sort}
    {paginate};",
        from = sort.get_from(),
        object_uuids = get_vec_in_string(object_uuids),
        sort = sort.get_complete(),
        paginate = paginate.get_complete(),
    );
    debug!("SQL objects order query: {}", query);

    let temp: Vec<ObjectUuid> = diesel::sql_query(query).load(conn).map_err(|err| {
        debug!("Failed to sort uuid: {:?}", err);
        ServiceError::InternalServerError
    })?;
    Ok(ObjectUuid::get_uuids(&temp))
}
