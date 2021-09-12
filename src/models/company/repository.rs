use crate::errors::ServiceResult;
use crate::models::company::model::{Company, ShowCompanyShort, CompanyAndRelatedData};
use crate::models::company::company_represent::model::CompanyRepresentAndRelatedData;
use crate::models::company::certificate::model::CertificateWithSlimFile;
use crate::models::company::company_type::model::CompanyTypeTranslateList;
use crate::models::company::company_fav::model::CompanyFav;
use crate::models::company::spec::model::CompanySpecWithTranslation;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::models::relate_ref::file::model::SlimFile;
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl Company {
    /// Get company data from company_ref table by uuid
    pub fn get_company_by_uuid(
        target_company_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Company> {
        Ok(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid))
            .first::<Company>(conn)?)
    }
}

impl ShowCompanyShort {
    /// Gets company short data by company uuid
    pub fn get_by_uuid(
        target_company_uuid: &Uuid,
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ShowCompanyShort> {
        // get target company
        let company: Company = Company::get_company_by_uuid(
            target_company_uuid,
            conn
        )
        .expect("Error loading company");

        // get image file (favicon) for company
        let image_file = SlimFile::get_file_by_uuid(&company.image_file_uuid, conn)
            .expect("Error loading company file");

        // get region for company
        let region_with_translate: RegionTranslateList = RegionTranslateList::get_region_by_id(
            &company.region_id,
            set_lang_id,
            conn
        ).expect("Error loading company_type");

        // get company type with translation for company
        let company_type_with_translate: CompanyTypeTranslateList = CompanyTypeTranslateList::get_company_type_by_id(
            &company.company_type_id,
            set_lang_id,
            conn
        ).expect("Error loading company_type");

        // check whether the object is being tracked auth user
        let is_followed = crate::models::company::company_fav::util::check_subscriber_by_uuid(
            target_company_uuid,
            target_user_uuid,
            conn
        ).expect("Error get value is_followed");

        Ok(ShowCompanyShort {
            uuid: company.uuid,
            shortname: company.shortname,
            description: company.description,
            inn: company.inn,
            image_file,
            region: region_with_translate,
            company_type: company_type_with_translate,
            is_followed,
            is_supplier: company.is_supplier,
            updated_at: company.updated_at,
        })
    }

    /// Gets companies short data by vec uuids
    pub fn get_list_by_uuids(
        target_companies_uuids: &[Uuid],
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        // the result for store the result :)
        let mut result: Vec<ShowCompanyShort> = Vec::new();

        // collecting data for each company
        for target_company_uuid in target_companies_uuids.iter() {
            result.push(ShowCompanyShort::get_by_uuid(
                target_company_uuid,
                target_user_uuid,
                set_lang_id,
                conn
            )?);
        }
        Ok(result)
    }
}

impl CompanyAndRelatedData {
    /// Collecting company data and related data using uuid
    pub fn collect_related_data(
        target_company_uuid: &Uuid,
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<CompanyAndRelatedData> {
        // collect data for company
        let company: Company = Company::get_company_by_uuid(
            target_company_uuid,
            conn
        ).expect("Error loading company");

        // get company owner
        let owner_user = crate::models::user::model::ShowUserShort::get_by_uuid(
            &company.user_uuid,
            conn
        ).expect("Error loading slim_user");

        // get image file (favicon) for company
        let image_file = SlimFile::get_file_by_uuid(&company.image_file_uuid, conn)
            .expect("Error loading company file");

        // get company represents for company
        let company_represents_with_related_data = CompanyRepresentAndRelatedData::get_list_represents_by_company_uuid(
            &company.uuid,
            set_lang_id,
            conn
        ).expect("Error loading company represents");

        // get region for company
        let region_with_translate: RegionTranslateList = RegionTranslateList::get_region_by_id(
            &company.region_id,
            set_lang_id,
            conn
        ).expect("Error loading company_type");

        // get company type with translation for company
        let company_type_with_translate: CompanyTypeTranslateList = CompanyTypeTranslateList::get_company_type_by_id(
            &company.company_type_id,
            set_lang_id,
            conn
        ).expect("Error loading company_type");

        // check whether the object is being tracked auth user
        let is_followed = crate::models::company::company_fav::util::check_subscriber_by_uuid(
            target_company_uuid,
            target_user_uuid,
            conn
        ).expect("Error get value is_followed");

        // count subscribers company
        let company_subscribers_count: i32 = CompanyFav::get_count_followers_by_uuid(&company.uuid, conn)?;

        // get certificates with slimfile for company
        let certificates_with_slimfile: Vec<CertificateWithSlimFile> = CertificateWithSlimFile::for_company(
            &company,
            conn
        ).expect("Error loading spec company with translate");

        // get specs with translation for company
        let company_specs_with_translate: Vec<CompanySpecWithTranslation> = CompanySpecWithTranslation::for_company(
            &company,
            set_lang_id,
            conn
        ).expect("Error loading spec company with translate");

        let result = CompanyAndRelatedData {
            uuid: company.uuid,
            orgname: company.orgname,
            shortname: company.shortname,
            inn: company.inn,
            phone: company.phone,
            email: company.email,
            description: company.description,
            address: company.address,
            site_url: company.site_url,
            time_zone: company.time_zone,
            owner_user,
            image_file,
            company_represents: company_represents_with_related_data,
            region: region_with_translate,
            company_type: company_type_with_translate,
            company_certificates: certificates_with_slimfile,
            company_specs: company_specs_with_translate,
            is_supplier: company.is_supplier,
            is_email_verified: company.is_email_verified,
            subscribers: company_subscribers_count,
            is_followed,
            is_enabled: company.is_enabled,
            is_delete: company.is_delete,
            created_at: company.created_at,
            updated_at: company.updated_at,
        };

        Ok(result)
    }
}
