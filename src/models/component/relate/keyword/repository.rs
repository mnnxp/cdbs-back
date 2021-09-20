use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::keyword::model::ComponentKeyword;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::schema::keyword_to_component::dsl as keyword_to_component;
use diesel::prelude::*;

impl Keyword {
    /// Get list keywords for component
    pub fn get_by_component(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Keyword>> {
        let target_vec_keyword_id: Vec<i32> = ComponentKeyword::belonging_to(component)
            .select(keyword_to_component::keyword_id)
            .load::<i32>(conn)?;
        Keyword::get_keyword_by_vec_id(&target_vec_keyword_id, conn)
    }
}
