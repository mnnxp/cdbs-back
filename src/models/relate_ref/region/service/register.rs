use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::relate_ref::region::model::{
    InsertableRegionTranslateList,
    IptRegionTranslateListData,
    RegionTranslateList,
    Region
};
use diesel::prelude::*;

pub(crate) fn create_region(
    new_region_data: IptRegionTranslateListData,
    conn: &PgConnection
) -> ServiceResult<RegionTranslateList> {
    use crate::schema::region_translate_list::dsl::*;

    let flag_found_region = region_translate_list
        .filter(id_lang.eq(&new_region_data.id_lang))
        .filter(region.eq(&new_region_data.region))
        .select(id_region)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_region START SEARCH ={:?}", flag_found_region);

    match flag_found_region {
        0 => {
            let new_id_region = {
                use crate::schema::region_ref::dsl::*;

                let new_region: Region = diesel::insert_into(region_ref)
                    .default_values()
                    .get_result(conn)?;

                new_region.id
            };

            let new_region_data = InsertableRegionTranslateList {
                id_region: new_id_region,
                id_lang: new_region_data.id_lang,
                region: new_region_data.region,
            };
            let inserted_region_data: RegionTranslateList = diesel::insert_into(region_translate_list)
                .values(&new_region_data)
                .get_result(conn)?;
            Ok(inserted_region_data)
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This region name is already there. Id: {}", flag_found_region))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
