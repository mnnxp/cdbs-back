/// Counts enabled subscribers in the specified table filtered by UUID.
///
/// Usage:
/// ```rust
/// let count = subscribers_count!(component_fav, component_uuid, target_component_uuid, conn)?;
/// ```
///
/// - `table`: the table module (e.g., `component_fav`)
/// - `column`: the UUID column in the table (e.g., `component_uuid`)
/// - `filter_uuid`: the UUID value to filter by (e.g., `target_component_uuid`)
/// - `conn`: database connection
///
/// Returns `Ok(count)` as `i32` or an error.
#[macro_export]
macro_rules! subscribers_count {
    ($table:ident, $column:ident, $filter_uuid:expr, $conn:expr) => {{
        let count: i64 = $table::$table
            .filter(
                $table::$column
                    .eq($filter_uuid)
                    .and($table::is_enabled.eq(true)),
            )
            .count()
            .get_result($conn)
            .map_err(|err| {
                debug!("Failed count for {}: {:?}", stringify!($table), err);
                ServiceError::InternalServerError
            })?;
        debug!("Subscribers count for {}: {:?}", stringify!($table), count);
        Ok(count as i32)
    }};
}
