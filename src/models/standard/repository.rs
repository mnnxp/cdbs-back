use crate::errors::ServiceResult;
use crate::models::standard::model::{Standard, ShowStandardShort, StandardAndRelatedData};
use crate::models::standard::standard_status::model::StandardStatusTranslateList;
use crate::models::standard::standard_fav::model::StandardFav;
use crate::models::standard::spec::model::StandardSpecWithTranslation;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::relate_ref::file::model::ShowFile;
use crate::schema::standard_ref::dsl as standard_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl Standard {
    /// Get standard data from standard_ref table by uuid
    pub fn get_standard_by_uuid(
        target_standard_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Standard> {
        Ok(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid))
            .first::<Standard>(conn)?)
    }
}

impl ShowStandardShort {
    pub fn get_list_by_uuids(
        target_standards_uuids: &[Uuid],
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        // the for collect the result :)
        let mut result: Vec<ShowStandardShort> = Vec::new();

        // collecting data for each standard
        for target_standard_uuid in target_standards_uuids.iter() {
            // get target standard
            let standard: Standard = Standard::get_standard_by_uuid(
                target_standard_uuid,
                conn
            ).expect("Error loading standard");

            // get standard owner company
            let owner_company = crate::models::company::model::ShowCompanyShort::get_by_uuid(
                &standard.company_uuid,
                target_user_uuid,
                set_lang_id,
                conn
            ).expect("Error loading company short data");

            // get standard type with translation for standard
            let standard_status: StandardStatusTranslateList = StandardStatusTranslateList::get_standard_status_by_id(
                &standard.standard_status_id,
                set_lang_id,
                conn
            ).expect("Error loading standard_status");

            // check whether the object is being tracked auth user
            let is_followed = crate::models::standard::standard_fav::util::check_subscriber_by_uuid(
                target_standard_uuid,
                target_user_uuid,
                conn
            ).expect("Error get is_followed");

            result.push(ShowStandardShort {
                uuid: standard.uuid,
                classifier: standard.classifier,
                name: standard.name,
                description: standard.description,
                specified_tolerance: standard.specified_tolerance,
                publication_at: standard.publication_at,
                owner_company,
                standard_status,
                updated_at: standard.updated_at,
                is_followed,
            });
        }
        Ok(result)
    }
}

impl StandardAndRelatedData {
    /// Collecting standard data and related data using uuid
    pub fn collect_related_data(
        target_standard_uuid: &Uuid,
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<StandardAndRelatedData> {
        // collect data for standard
        let standard: Standard = Standard::get_standard_by_uuid(
            target_standard_uuid,
            conn
        ).expect("Error loading standard");

        // get image file (favicon) for standard
        let image_file = ShowFile::get_file_by_uuid(&standard.image_file_uuid, conn)
            .expect("Error loading standard file");

        // get standard owner user
        let owner_user = crate::models::user::model::ShowUserShort::get_by_uuid(
            &standard.user_uuid,
            conn
        ).expect("Error loading slim_user");

        // get standard owner company
        let owner_company = crate::models::company::model::ShowCompanyShort::get_by_uuid(
            &standard.company_uuid,
            target_user_uuid,
            set_lang_id,
            conn
        ).expect("Error loading company short data");

        // todo!(need make access manager)
        // get standard type with translation for standard
        // let type_access: TypeAccessTranslateList = TypeAccessTranslateList::get_standard_status_by_id(
        //     &standard.type_access_id,
        //     set_lang_id,
        //     conn
        // ).expect("Error loading type_access");

        // get standard type with translation for standard
        let standard_status: StandardStatusTranslateList = StandardStatusTranslateList::get_standard_status_by_id(
            &standard.standard_status_id,
            set_lang_id,
            conn
        ).expect("Error loading standard_status");

        // get region for company
        let region: RegionTranslateList = RegionTranslateList::get_region_by_id(
            &standard.region_id,
            set_lang_id,
            conn
        ).expect("Error loading company_type");

        // count subscribers standard
        let subscribers: i32 = StandardFav::get_count_followers_by_uuid(&standard.uuid, conn)?;

        // get files for standard
        let standard_files = ShowFile::for_standard(&standard, conn)
            .expect("Error loading standard files");

        // get specs with translation for standard
        let standard_specs: Vec<StandardSpecWithTranslation> = StandardSpecWithTranslation::for_standard(
            &standard,
            set_lang_id,
            conn
        ).expect("Error loading spec standard with translate");

        // check whether the object is being tracked auth user
        let is_followed = crate::models::standard::standard_fav::util::check_subscriber_by_uuid(
            target_standard_uuid,
            target_user_uuid,
            conn
        ).expect("Error get is_followed");

        // get keywords for standard
        let standard_keywords: Vec<Keyword> = Keyword::get_by_standard(
            &standard,
            conn
        ).expect("Error loading standard keywords");

        let result = StandardAndRelatedData {
            uuid: standard.uuid,
            parent_standard_uuid: standard.parent_standard_uuid,
            classifier: standard.classifier,
            name: standard.name,
            description: standard.description,
            specified_tolerance: standard.specified_tolerance,
            technical_committee: standard.technical_committee,
            publication_at: standard.publication_at,
            image_file,
            owner_user,
            owner_company,
            type_access_id: standard.type_access_id,
            standard_status,
            region,
            is_delete: standard.is_delete,
            created_at: standard.created_at,
            updated_at: standard.updated_at,
            standard_files,
            standard_specs,
            standard_keywords,
            subscribers,
            is_followed,
        };

        Ok(result)
    }
}
