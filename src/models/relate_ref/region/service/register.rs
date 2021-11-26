use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::region::model::{
    InsertableRegionTranslateList,
    IptRegionTranslateListData,
    RegionTranslateList,
    Region
};
use diesel::{PgConnection, prelude::*};

pub(crate) fn create_region(
    new_region_data: &IptRegionTranslateListData,
    conn: &PgConnection
) -> ServiceResult<RegionTranslateList> {
    use crate::schema::region_translate_list::dsl::*;

    let flag_found_region = region_translate_list
        .filter(lang_id.eq(&new_region_data.lang_id)
        .and(region.eq(&new_region_data.region)))
        .select(region_id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_region START SEARCH ={:?}", flag_found_region);

    match flag_found_region {
        0 => {
            let new_region_id = {
                use crate::schema::region_ref::dsl::*;

                let new_region: Region = diesel::insert_into(region_ref)
                    .default_values()
                    .get_result(conn)
                    .map_err(|err| {
                        debug!("Failed insert region: {:?}", err);
                        ServiceError::InternalServerError
                    })?;

                new_region.id
            };

            let new_region_data = InsertableRegionTranslateList {
                region_id: new_region_id,
                lang_id: new_region_data.lang_id,
                region: new_region_data.region.clone(),
            };
            diesel::insert_into(region_translate_list)
                .values(&new_region_data)
                .get_result::<RegionTranslateList>(conn)
                .map_err(|err| {
                    debug!("Failed get program: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This region name is already there. Id: {}", flag_found_region))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
