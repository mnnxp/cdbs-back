use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::schema::region_translate_list::dsl as region_translate_list;
use diesel::{PgConnection, prelude::*};

impl RegionTranslateList {
    pub(crate) fn get_region_by_id(
        target_region_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<RegionTranslateList> {
        let region = region_translate_list::region_translate_list
            .filter(region_translate_list::region_id.eq(target_region_id)
            .and(region_translate_list::lang_id.eq(set_lang_id)))
            .first::<RegionTranslateList>(conn);

        // if not found data for set lang
        match region {
            Ok(rn) => Ok(rn),
            Err(err) => {
                debug!("Not found set lang for region: {:?}", err);
                region_translate_list::region_translate_list
                    .filter(region_translate_list::region_id.eq(target_region_id))
                    .first::<RegionTranslateList>(conn)
                    .map_err(|err| {
                        debug!("Failed insert region: {:?}", err);
                        ServiceError::InternalServerError
                    })
            },
        }
    }
}
