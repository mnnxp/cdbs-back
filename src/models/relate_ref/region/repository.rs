use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::schema::region_translate_list::dsl as region_translate_list;
use diesel::{prelude::*, PgConnection};

impl RegionTranslateList {
    pub(crate) fn get_region_by_id(
        target_region_id: &i32,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<RegionTranslateList> {
        let check_region = region_translate_list::region_translate_list
            .filter(
                region_translate_list::region_id
                    .eq(target_region_id)
                    .and(region_translate_list::lang_id.eq(set_lang_id)),
            )
            .limit(1)
            .load::<RegionTranslateList>(conn)
            .map_err(|err| {
                debug!("Failed check region: {:?}", err);
                ServiceError::InternalServerError
            })?;

        match check_region.first() {
            Some(x) => Ok(x.clone()),
            None => region_translate_list::region_translate_list
                .filter(region_translate_list::region_id.eq(target_region_id))
                .first::<RegionTranslateList>(conn)
                .map_err(|err| {
                    debug!("Failed insert region: {:?}", err);
                    ServiceError::InternalServerError
                }),
        }
    }
}
