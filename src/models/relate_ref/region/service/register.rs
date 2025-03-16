use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::relate_ref::region::model::{
    InsertableRegionTranslateList, IptRegionTranslateListData, RegionTranslateList
};
use crate::schema::region_ref::dsl as region_ref;
use crate::schema::region_translate_list::dsl as region_translate_list;
use diesel::{PgConnection, prelude::*};

/// Добавляет наименование региона. Возвращает идентификатор и наименование региона.
/// Возвращает ошибку с идентификатором региона, если он уже существует.
pub(crate) fn create_region(
    new_region_data: &IptRegionTranslateListData,
    conn: &mut PgConnection
) -> ServiceResult<RegionTranslateList> {
    let flag_found = region_translate_list::region_translate_list
        .filter(region_translate_list::lang_id.eq(&new_region_data.lang_id)
        .and(region_translate_list::region.eq(&new_region_data.region)))
        .select(region_translate_list::region_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed check region: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match flag_found.first() {
        Some(x) => {
            Err(get_err_msg(ErrorMessage::NameAlreadyThereX("region".to_string(), *x)))
        }
        None => {
            let new_region_id = diesel::insert_into(region_ref::region_ref)
                .default_values()
                .returning(region_ref::id)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed insert region: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            let new_region_data = InsertableRegionTranslateList {
                region_id: new_region_id,
                lang_id: new_region_data.lang_id,
                region: new_region_data.region.clone(),
            };

            diesel::insert_into(region_translate_list::region_translate_list)
                .values(&new_region_data)
                .get_result::<RegionTranslateList>(conn)
                .map_err(|err| {
                    debug!("Failed get program: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
    }
}
