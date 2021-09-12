use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::keyword::model::{
    KeywordComponent,
    IptKeywordComponentData,
    InsertableKeywordComponent
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn add_component_keyword(
    data: IptKeywordComponentData,
    conn: &PgConnection
) -> ServiceResult<KeywordComponent> {
    use crate::schema::keyword_to_component::dsl::*;

    let new_component_keyword: InsertableKeywordComponent = data.into();

    let flag_found_keyword = keyword_to_component
        .filter(component_uuid.eq(&new_component_keyword.component_uuid)
        .and(keyword_id.eq(&new_component_keyword.keyword_id)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_keyword START SEARCH ={:?}", flag_found_keyword);

    match flag_found_keyword as i32 {
        0 => {
            let inserted_component_keyword: KeywordComponent = diesel::insert_into(keyword_to_component)
                .values(&new_component_keyword)
                .get_result(conn)?;
            Ok(inserted_component_keyword)
        },
        _ => Err(ServiceError::BadRequest("This keyword name is already with the component.".to_string())),
    }
}
