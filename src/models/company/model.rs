use super::company_represent::model::CompanyRepresentAndRelatedData;
use super::certificate::model::CompanyCertificateAndFile;
use super::company_type::model::CompanyTypeTranslateList;
use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::{
    file::model::DownloadFile,
    spec::model::SpecTranslateList,
    region::model::RegionTranslateList,
    type_access::model::TypeAccessTranslateList,
};
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[primary_key(uuid)]
#[table_name = "company_ref"]
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

#[derive(Debug, SimpleObject)]
pub(crate) struct CompanyAndRelatedData {
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
    pub(crate) owner_user: ShowUserShort,
    pub(crate) image_file: DownloadFile,
    pub(crate) region: RegionTranslateList,
    pub(crate) company_represents: Vec<CompanyRepresentAndRelatedData>,
    pub(crate) company_type: CompanyTypeTranslateList,
    // show certificates company
    pub(crate) company_certificates: Vec<CompanyCertificateAndFile>,
    pub(crate) company_specs: Vec<SpecTranslateList>,
    pub(crate) type_access: TypeAccessTranslateList,
    pub(crate) is_supplier: bool,
    pub(crate) is_email_verified: bool,
    // count users to folloded the company
    pub(crate) subscribers: i32,
    // for display the checkbox "favorites"
    pub(crate) is_followed: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, SimpleObject)]
pub(crate) struct ShowCompanyShort {
    pub(crate) uuid: Uuid,
    pub(crate) shortname: String,
    pub(crate) inn: String,
    pub(crate) description: String,
    pub(crate) image_file: DownloadFile,
    pub(crate) region: RegionTranslateList,
    pub(crate) company_type: CompanyTypeTranslateList,
    pub(crate) is_supplier: bool,
    // for display the checkbox "favorites"
    pub(crate) is_followed: bool,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "company_ref"]
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
        self.image_file_uuid = Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297").unwrap();
    }
}

#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptCompanyData {
    pub(crate) orgname: String,
    pub(crate) shortname: String,
    pub(crate) inn: String,
    pub(crate) phone: String,
    pub(crate) email: String,
    pub(crate) description: String,
    pub(crate) address: String,
    pub(crate) site_url: String,
    pub(crate) time_zone: String,
    pub(crate) region_id: i32,
    pub(crate) company_type_id: i32,
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

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateCompanyData {
    pub(crate) orgname: Option<String>,
    pub(crate) shortname: Option<String>,
    pub(crate) inn: Option<String>,
    pub(crate) phone: Option<String>,
    pub(crate) email: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) site_url: Option<String>,
    pub(crate) time_zone: Option<String>,
    pub(crate) region_id: Option<i32>,
    pub(crate) company_type_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, Default, SimpleObject)]
pub(crate) struct SlimCompany {
    pub(crate) uuid: Uuid,
    pub(crate) shortname: String,
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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptCompaniesArg {
    pub(crate) companies_uuids:  Option<Vec<Uuid>>,
    pub(crate) user_uuid: Option<Uuid>,
    pub(crate) favorite: Option<bool>,
    pub(crate) supplier: Option<bool>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct CompaniesArg {
    pub(crate) filter_companies_uuids: Vec<Uuid>,
    pub(crate) user_uuid: Option<Uuid>,
    pub(crate) favorite: bool,
    pub(crate) supplier: bool,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for CompaniesArg {
    fn default() -> Self {
        Self {
            filter_companies_uuids: Vec::new(),
            user_uuid: None,
            favorite: false,
            supplier: false,
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptCompaniesArg> for CompaniesArg {
    fn from(data: IptCompaniesArg) -> Self {
        let IptCompaniesArg {
            companies_uuids,
            user_uuid,
            favorite,
            supplier,
            limit,
            offset,
        } = data;

        Self {
            filter_companies_uuids: companies_uuids.unwrap_or_default(),
            user_uuid,
            favorite: favorite.unwrap_or(false),
            supplier: supplier.unwrap_or(false),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
