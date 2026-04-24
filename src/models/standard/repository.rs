use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::{ServiceError, ServiceResult};
use crate::graphql::file::ShowFileRelatedData;
use crate::graphql::standard_model::{ShowStandardShort, StandardAndRelatedData};
use crate::models::company::model::ShowCompanyShort;
use crate::models::relate_ref::{
    file::model::DownloadFile, keyword::model::Keyword, spec::model::SpecTranslateList,
    type_access::model::TypeAccessTranslateList,
};
use crate::models::search::{
    model::ExtraOptions,
    order::{Paginate, Sort, TableName},
};
use crate::models::standard::{
    model::Standard,
    standard_fav::model::StandardFav, standard_fav::util::check_subscriber_by_uuid,
    standard_status::model::StandardStatusTranslateList,
};
use crate::schema::standard_ref::dsl as standard_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl Standard {
    /// Get standard data from standard_ref table by uuid
    pub(crate) fn get_standard_by_uuid(
        target_standard_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<Standard> {
        standard_ref::standard_ref
            .filter(
                standard_ref::uuid
                    .eq(target_standard_uuid)
                    .and(standard_ref::is_delete.eq(false)),
            )
            .select((
                standard_ref::uuid,
                standard_ref::parent_standard_uuid,
                standard_ref::name,
                standard_ref::description,
                standard_ref::publication_at,
                standard_ref::image_file_uuid,
                standard_ref::user_uuid,
                standard_ref::company_uuid,
                standard_ref::type_access_id,
                standard_ref::standard_status_id,
                standard_ref::created_at,
                standard_ref::updated_at,
            ))
            .first::<Standard>(conn)
            .map_err(|err| {
                debug!("Failed get standard: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl ShowStandardShort {
    /// Gets standards by filter or all public
    /// limit and offset works only without filter
    pub(crate) fn get_standards(
        filter_standards_uuids: &[Uuid],
        options: &ExtraOptions,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        match filter_standards_uuids.is_empty() {
            true => ShowStandardShort::get_all_public(options, paginate, conn),
            false => ShowStandardShort::get_list_by_uuids(filter_standards_uuids, options, conn),
        }
    }

    /// Gets standard short data by standard_uuid with check access
    pub(crate) fn get_by_uuid(
        target_standard_uuid: &Uuid,
        options: &ExtraOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowStandardShort> {
        require_permission(
            &options.logged_user_uuid,
            AccessEntity::Standard,
            target_standard_uuid,
            AccessOperation::Read,
            conn,
        )?;

        // get target standard
        let standard: Standard = Standard::get_standard_by_uuid(target_standard_uuid, conn)
            .expect("Error loading standard");

        // get image file (favicon) for standard
        let image_file = DownloadFile::get_by_file_uuid(&standard.image_file_uuid, &options.domain, conn)
            .expect("Error get presigned url main image");

        // get standard owner company
        let owner_company = ShowCompanyShort::get_without_check_by_uuid(
            &standard.company_uuid,
            options,
            conn,
        )
        .expect("Error loading company short data");

        // get standard type with translation for standard
        let standard_status = StandardStatusTranslateList::get_by_id(
            standard.standard_status_id,
            options.set_lang_id,
            conn,
        )
        .expect("Error loading standard_status");

        // check whether the object is being tracked auth user
        let is_followed =
            check_subscriber_by_uuid(target_standard_uuid, &options.logged_user_uuid, conn)
                .expect("Error get is_followed");

        Ok(ShowStandardShort {
            uuid: standard.uuid,
            name: standard.name,
            description: standard.description,
            publication_at: standard.publication_at,
            image_file,
            owner_company,
            standard_status,
            updated_at: standard.updated_at,
            is_followed,
        })
    }

    pub(crate) fn get_list_by_uuids(
        standard_uuids: &[Uuid],
        options: &ExtraOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        // the for collect the result :)
        let mut result: Vec<ShowStandardShort> = Vec::new();
        // collecting data for each standard
        for target_standard_uuid in standard_uuids.iter() {
            let _res = ShowStandardShort::get_by_uuid(target_standard_uuid, options, conn)
                .map(|value| result.push(value))
                .map_err(|err| debug!("Failed get standard data: {:?}", err));
        }
        Ok(result)
    }

    /// Gets all public standards short data
    pub(crate) fn get_all_public(
        options: &ExtraOptions,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        // gets all public standards uuids
        let standard_uuids = standard_ref::standard_ref
            .filter(
                standard_ref::type_access_id
                    .eq(3)
                    .and(standard_ref::is_delete.eq(false)),
            )
            .select(standard_ref::uuid)
            .limit(paginate.limit)
            .offset(paginate.offset)
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed get public standards: {:?}", err);
                ServiceError::InternalServerError
            })?;

        // the for collect the result :)
        let mut result: Vec<ShowStandardShort> = Vec::new();
        // collecting data for each standard
        for target_standard_uuid in standard_uuids.iter() {
            let _res = ShowStandardShort::get_by_uuid(target_standard_uuid, options, conn)
                .map(|value| result.push(value))
                .map_err(|err| debug!("Failed get standard data: {:?}", err));
        }
        Ok(result)
    }
}

impl StandardAndRelatedData {
    /// Gathers data for a standard and related data by uuid
    pub(crate) fn collect_related_data(
        target_standard_uuid: &Uuid,
        options: &ExtraOptions,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<StandardAndRelatedData> {
        require_permission(
            &options.logged_user_uuid,
            AccessEntity::Standard,
            target_standard_uuid,
            AccessOperation::Read,
            conn,
        )?;

        // collect data for standard
        let standard: Standard = Standard::get_standard_by_uuid(target_standard_uuid, conn)
            .expect("Error loading standard");

        // get image file (favicon) for standard
        let image_file = DownloadFile::get_by_file_uuid(&standard.image_file_uuid, &options.domain, conn)
            .expect("Error get presigned url main image");

        // get data a owner user for a standard
        let owner_user = crate::models::user::model::ShowUserShort::get_without_check_by_uuid(
            &standard.user_uuid,
            &options.domain,
            conn,
        )
        .expect("Error loading slim_user");

        // get data a owner company for a standard
        let owner_company = ShowCompanyShort::get_without_check_by_uuid(
            &standard.company_uuid,
            options,
            conn,
        )
        .expect("Error loading company short data");

        // get standard type with translation for standard
        let type_access = TypeAccessTranslateList::get_type_access_by_id(
            standard.type_access_id,
            options.set_lang_id,
            conn,
        )
        .expect("Error loading type_access");

        // get standard type with translation for standard
        let standard_status = StandardStatusTranslateList::get_by_id(
            standard.standard_status_id,
            options.set_lang_id,
            conn,
        )
        .expect("Error loading standard_status");

        // count subscribers standard
        let subscribers = StandardFav::get_count_followers_by_uuid(&standard.uuid, conn)
            .expect("Error loading subscribers");

        // get files for standard
        let standard_files = ShowFileRelatedData::for_standard_by_uuid(
            &standard.uuid,
            &Sort::parsing(TableName::FileRef, "", false),
            paginate,
            &options.domain,
            conn,
        )
        .expect("Error loading standard files");

        // get specs with translation for standard
        let standard_specs: Vec<SpecTranslateList> = SpecTranslateList::for_standard_by_uuid(
            &standard.uuid,
            options.set_lang_id,
            paginate,
            conn,
        )
        .expect("Error loading spec standard with translate");

        // check whether the object is being tracked auth user
        let is_followed =
            check_subscriber_by_uuid(target_standard_uuid, &options.logged_user_uuid, conn)
                .expect("Error get is_followed");

        // get keywords for standard
        let standard_keywords: Vec<Keyword> =
            Keyword::for_standard_by_uuid(&standard.uuid, paginate, conn)
                .expect("Error loading standard keywords");

        Ok(StandardAndRelatedData {
            uuid: standard.uuid,
            parent_standard_uuid: standard.parent_standard_uuid,
            name: standard.name,
            description: standard.description,
            publication_at: standard.publication_at,
            image_file,
            owner_user,
            owner_company,
            type_access,
            standard_status,
            created_at: standard.created_at,
            updated_at: standard.updated_at,
            standard_files,
            standard_specs,
            standard_keywords,
            subscribers,
            is_followed,
        })
    }
}
