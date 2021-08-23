use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::keyword::model::KeywordComponent;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::schema::keyword_ref::dsl as keyword_ref;
use crate::schema::keyword_to_component::dsl as keyword_to_component;
use diesel::prelude::*;

impl Keyword {
    pub fn get_keyword_by_id(
        target_id_keyword: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Keyword> {
        Ok(keyword_ref::keyword_ref
            .filter(keyword_ref::id.eq(target_id_keyword))
            .first::<Keyword>(conn)?)
    }

    pub fn get_keyword_by_vec_id(
        target_vec_id_keyword: &[i32],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Keyword>> {
        Ok(keyword_ref::keyword_ref
            .filter(keyword_ref::id.eq_any(target_vec_id_keyword))
            .load::<Keyword>(conn)?)
    }

    pub fn for_component(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Keyword>> {
        let target_id_keyword: Vec<i32> = KeywordComponent::belonging_to(component)
            .select(keyword_to_component::id_keyword)
            .load::<i32>(conn)?;
        Ok(keyword_ref::keyword_ref
            .filter(keyword_ref::id.eq_any(target_id_keyword))
            .load::<Keyword>(conn)?)
    }
}
