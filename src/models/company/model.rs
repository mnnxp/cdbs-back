use super::certificate::model::CompanyCertificateAndFile;
use super::company_represent::model::CompanyRepresentAndRelatedData;
use super::company_type::model::CompanyTypeTranslateList;
use crate::models::relate_ref::{
    file::model::DownloadFile, file::util::get_default_image, region::model::RegionTranslateList,
    spec::model::SpecTranslateList, type_access::model::TypeAccessTranslateList,
};
use crate::models::user::model::ShowUserShort;
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = company_ref)]
pub(crate) struct Company {
    pub(crate) uuid: Uuid,
    pub(crate) orgname: String,
    pub(crate) shortname: String,
    pub(crate) inn: String,
    pub(crate) phone: String,
    pub(crate) email: String,
    pub(crate) description: String,
    pub(crate) address: String,
    pub(crate) site_url: String,
    pub(crate) time_zone: String,
    pub(crate) user_uuid: Uuid,
    pub(crate) image_file_uuid: Uuid,
    pub(crate) region_id: i32,
    pub(crate) company_type_id: i32,
    pub(crate) type_access_id: i32,
    pub(crate) is_supplier: bool,
    pub(crate) is_email_verified: bool,
    // pub(crate) is_enabled: bool,
    // pub(crate) is_delete: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Full company information and related data
#[derive(Debug, SimpleObject)]
pub(crate) struct CompanyAndRelatedData {
    /// Company UUID on the platform
    pub(crate) uuid: Uuid,
    /// Company name
    pub(crate) orgname: String,
    /// Abbreviated name
    pub(crate) shortname: String,
    /// TIN or other tax identifier of the company
    pub(crate) inn: String,
    /// Phone number
    pub(crate) phone: String,
    /// Company e-mail
    pub(crate) email: String,
    /// Company Description
    pub(crate) description: String,
    /// Company address
    pub(crate) address: String,
    /// Company website
    pub(crate) site_url: String,
    /// Main time zone
    pub(crate) time_zone: String,
    /// Data on the profile that owns the company
    pub(crate) owner_user: ShowUserShort,
    /// Data for displaying the company logo
    pub(crate) image_file: DownloadFile,
    /// Main company region
    pub(crate) region: RegionTranslateList,
    /// Data on the company's representative offices
    pub(crate) company_represents: Vec<CompanyRepresentAndRelatedData>,
    /// Type of company/society organization
    pub(crate) company_type: CompanyTypeTranslateList,
    /// List of certificates and competencies of the companies
    pub(crate) company_certificates: Vec<CompanyCertificateAndFile>,
    /// List of catalogs monitored by the company
    pub(crate) company_specs: Vec<SpecTranslateList>,
    /// Type of access to company profile
    pub(crate) type_access: TypeAccessTranslateList,
    /// Supplier status (within the platform)
    pub(crate) is_supplier: bool,
    /// E-mail confirmation result flag
    pub(crate) is_email_verified: bool,
    /// Number of people who have added the company to their bookmarks
    pub(crate) subscribers: i32,
    /// Flag of company presence in user's bookmarks
    pub(crate) is_followed: bool,
    /// Date of creation of the company profile
    pub(crate) created_at: NaiveDateTime,
    /// Date of update of the company's basic data
    pub(crate) updated_at: NaiveDateTime,
}

/// Abbreviated company data
#[derive(Debug, SimpleObject)]
pub(crate) struct ShowCompanyShort {
    /// Company UUID on the platform
    pub(crate) uuid: Uuid,
    /// Abbreviated name
    pub(crate) shortname: String,
    /// TIN or other tax identifier of the company
    pub(crate) inn: String,
    /// Company Description
    pub(crate) description: String,
    /// Data for displaying the company logo
    pub(crate) image_file: DownloadFile,
    /// Main region of the company's activity
    pub(crate) region: RegionTranslateList,
    /// Type of company/community organization
    pub(crate) company_type: CompanyTypeTranslateList,
    /// Supplier status (within the platform)
    pub(crate) is_supplier: bool,
    /// Flag of company presence in user's bookmarks
    pub(crate) is_followed: bool,
    /// Date of creation of the company profile
    pub(crate) created_at: NaiveDateTime,
    /// Date of update of the company's basic data
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_ref)]
pub(crate) struct InsertableCompany {
    uuid: Uuid,
    orgname: String,
    shortname: String,
    inn: String,
    phone: String,
    email: String,
    description: String,
    address: String,
    site_url: String,
    time_zone: String,
    user_uuid: Uuid,
    image_file_uuid: Uuid,
    region_id: i32,
    company_type_id: i32,
    type_access_id: i32,
    is_supplier: bool,
    is_email_verified: bool,
    is_enabled: bool,
    is_delete: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl InsertableCompany {
    /// Set user uuid (for set logged user as owner)
    pub(crate) fn set_user_uuid(&mut self, user_uuid: &Uuid) {
        self.user_uuid = *user_uuid;
    }

    /// Set image uuid (for set default image)
    pub(crate) fn set_image_uuid(&mut self) {
        self.image_file_uuid = get_default_image();
    }
}

/// New company data
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptCompanyData {
    /// Company name
    pub(crate) orgname: String,
    /// Abbreviated name
    pub(crate) shortname: String,
    /// TIN or other tax identifier of the company
    pub(crate) inn: String,
    /// Phone number
    pub(crate) phone: String,
    /// Company e-mail
    pub(crate) email: String,
    /// Company Description
    pub(crate) description: String,
    /// Company address
    pub(crate) address: String,
    /// Company website
    pub(crate) site_url: String,
    /// Main time zone
    pub(crate) time_zone: String,
    /// Identifier of the company's main region
    pub(crate) region_id: i32,
    /// Company/community organization type identifier
    pub(crate) company_type_id: i32,
    /// Company profile access type identifier
    pub(crate) type_access_id: i32,
}

impl From<&IptCompanyData> for InsertableCompany {
    fn from(ipt_data: &IptCompanyData) -> Self {
        let IptCompanyData {
            orgname,
            shortname,
            inn,
            phone,
            email,
            description,
            address,
            site_url,
            time_zone,
            region_id,
            company_type_id,
            type_access_id,
        } = ipt_data;

        Self {
            uuid: Uuid::new_v4(),
            orgname: orgname.clone(),
            shortname: shortname.clone(),
            inn: inn.clone(),
            phone: phone.clone(),
            email: email.clone(),
            description: description.clone(),
            address: address.clone(),
            site_url: site_url.clone(),
            time_zone: time_zone.clone(),
            user_uuid: Uuid::nil(),
            image_file_uuid: Uuid::nil(),
            region_id: *region_id,
            company_type_id: *company_type_id,
            type_access_id: *type_access_id,
            is_supplier: false,
            is_email_verified: false,
            is_enabled: true,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

/// Data for updating the company profile.
/// The data is updated only for the provide values.
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateCompanyData {
    /// Company name
    pub(crate) orgname: Option<String>,
    /// Abbreviated name
    pub(crate) shortname: Option<String>,
    /// TIN or other tax identifier of the company
    pub(crate) inn: Option<String>,
    /// Phone number
    pub(crate) phone: Option<String>,
    /// Company e-mail
    pub(crate) email: Option<String>,
    /// Company Description
    pub(crate) description: Option<String>,
    /// Company address
    pub(crate) address: Option<String>,
    /// Company website
    pub(crate) site_url: Option<String>,
    /// Main time zone
    pub(crate) time_zone: Option<String>,
    /// Main company region
    pub(crate) region_id: Option<i32>,
    /// Type of company/society organization
    pub(crate) company_type_id: Option<i32>,
}

/// Minimum information about the company
#[derive(Debug, Serialize, Deserialize, Queryable, Clone, Default, SimpleObject)]
pub(crate) struct SlimCompany {
    /// Company UUID on the platform
    pub(crate) uuid: Uuid,
    /// Abbreviated name
    pub(crate) shortname: String,
    /// Supplier status (within the platform)
    pub(crate) is_supplier: bool,
}

impl From<Company> for SlimCompany {
    fn from(company: Company) -> Self {
        let Company {
            uuid,
            shortname,
            is_supplier,
            ..
        } = company;

        Self {
            uuid,
            shortname,
            is_supplier,
        }
    }
}

/// Arguments for companies data query
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptCompaniesArg {
    /// Filter by company UUID
    pub(crate) companies_uuids: Option<Vec<Uuid>>,
    /// Filter by company owner
    pub(crate) user_uuid: Option<Uuid>,
    /// Filter by availability of companies in user's favorites
    pub(crate) favorite: Option<bool>,
    /// Filter by supplier status
    pub(crate) supplier: Option<bool>,
}

#[derive(Debug)]
pub(crate) struct CompaniesArg {
    pub(crate) filter_companies_uuids: Vec<Uuid>,
    pub(crate) user_uuid: Option<Uuid>,
    pub(crate) favorite: bool,
    pub(crate) supplier: bool,
}

impl CompaniesArg {
    /// Returns a CompaniesArg with the given arguments and language
    pub(crate) fn by_arg(data: IptCompaniesArg) -> Self {
        let IptCompaniesArg {
            companies_uuids,
            user_uuid,
            favorite,
            supplier,
        } = data;
        Self {
            filter_companies_uuids: companies_uuids.unwrap_or_default(),
            user_uuid,
            favorite: favorite.unwrap_or(false),
            supplier: supplier.unwrap_or(false),
        }
    }

    /// Returns a CompaniesArg with the specified language and default arguments
    pub(crate) fn by_lang() -> Self {
        Self {
            filter_companies_uuids: Vec::new(),
            user_uuid: None,
            favorite: false,
            supplier: false,
        }
    }
}
