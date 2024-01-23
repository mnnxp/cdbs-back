use super::company_represent::model::CompanyRepresentAndRelatedData;
use super::certificate::model::CompanyCertificateAndFile;
use super::company_type::model::CompanyTypeTranslateList;
use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::{
    file::model::DownloadFile,
    file::util::get_default_image,
    spec::model::SpecTranslateList,
    region::model::RegionTranslateList,
    type_access::model::TypeAccessTranslateList,
};
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

/// Полная информация о компании и связанные с ней данные
#[derive(Debug, SimpleObject)]
pub(crate) struct CompanyAndRelatedData {
    /// Идентификатор компании на платформе
    pub(crate) uuid: Uuid,
    /// Наименование компании
    pub(crate) orgname: String,
    /// Сокращенное наименование
    pub(crate) shortname: String,
    /// ИНН или иной налоговый идентификатор компании
    pub(crate) inn: String,
    /// Номер телефона
    pub(crate) phone: String,
    /// Эл.почта компании
    pub(crate) email: String,
    /// Описание компании
    pub(crate) description: String,
    /// Адрес компании
    pub(crate) address: String,
    /// Сайт компании
    pub(crate) site_url: String,
    /// Основная временная зона
    pub(crate) time_zone: String,
    /// Данные о профиле владеющем компанией
    pub(crate) owner_user: ShowUserShort,
    /// Данные для отображения логотипа компании
    pub(crate) image_file: DownloadFile,
    /// Основной регион компании
    pub(crate) region: RegionTranslateList,
    /// Данные о представительствах компании
    pub(crate) company_represents: Vec<CompanyRepresentAndRelatedData>,
    /// Тип организации компании/общества
    pub(crate) company_type: CompanyTypeTranslateList,
    /// Список сертификатов и грамот компании
    pub(crate) company_certificates: Vec<CompanyCertificateAndFile>,
    /// Перечень отслеживаемых компанией каталогов
    pub(crate) company_specs: Vec<SpecTranslateList>,
    /// Тип доступа к профилю компании
    pub(crate) type_access: TypeAccessTranslateList,
    /// Статус поставщика (в рамках платформы)
    pub(crate) is_supplier: bool,
    /// Флаг результата подтверждения эл. почты
    pub(crate) is_email_verified: bool,
    /// Количество добавивших компанию в закладки
    pub(crate) subscribers: i32,
    /// Флаг наличия компаниии в закладках пользователя
    pub(crate) is_followed: bool,
    /// Дата создания профиля компании
    pub(crate) created_at: NaiveDateTime,
    /// Дата обновления основных данных компании
    pub(crate) updated_at: NaiveDateTime,
}

/// Сокращенные данные о компании
#[derive(Debug, SimpleObject)]
pub(crate) struct ShowCompanyShort {
    /// Идентификатор компании на платформе
    pub(crate) uuid: Uuid,
    /// Сокращенное наименование
    pub(crate) shortname: String,
    /// ИНН или иной налоговый идентификатор компании
    pub(crate) inn: String,
    /// Описание компании
    pub(crate) description: String,
    /// Данные для отображения логотипа компании
    pub(crate) image_file: DownloadFile,
    /// Основной регион деятельности компании
    pub(crate) region: RegionTranslateList,
    /// Тип организациии компании/общества
    pub(crate) company_type: CompanyTypeTranslateList,
    /// Статус поставщика (в рамках платформы)
    pub(crate) is_supplier: bool,
    /// Флаг наличия компаниии в закладках пользователя
    pub(crate) is_followed: bool,
    /// Дата обновления основных данных компании
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

/// Данные новой компании
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptCompanyData {
    /// Наименование компании
    pub(crate) orgname: String,
    /// Сокращенное наименование
    pub(crate) shortname: String,
    /// ИНН или иной налоговый идентификатор компании
    pub(crate) inn: String,
    /// Номер телефона
    pub(crate) phone: String,
    /// Эл.почта компании
    pub(crate) email: String,
    /// Описание компании
    pub(crate) description: String,
    /// Адрес компании
    pub(crate) address: String,
    /// Сайт компании
    pub(crate) site_url: String,
    /// Основная временная зона
    pub(crate) time_zone: String,
    /// Индентификатор основного региона компании
    pub(crate) region_id: i32,
    /// Индентификатор типа организации компании/общества
    pub(crate) company_type_id: i32,
    /// Индентификатор типа доступа к профилю компании
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

/// Данные для обновления профиля компании.
/// Обновление данных происходит только для заданных значений.
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateCompanyData {
    /// Наименование компании
    pub(crate) orgname: Option<String>,
    /// Сокращенное наименование
    pub(crate) shortname: Option<String>,
    /// ИНН или иной налоговый идентификатор компании
    pub(crate) inn: Option<String>,
    /// Номер телефона
    pub(crate) phone: Option<String>,
    /// Эл.почта компании
    pub(crate) email: Option<String>,
    /// Описание компании
    pub(crate) description: Option<String>,
    /// Адрес компании
    pub(crate) address: Option<String>,
    /// Сайт компании
    pub(crate) site_url: Option<String>,
    /// Основная временная зона
    pub(crate) time_zone: Option<String>,
    /// Основной регион компании
    pub(crate) region_id: Option<i32>,
    /// Тип организации компании/общества
    pub(crate) company_type_id: Option<i32>,
}

/// Минимальная информация о компании
#[derive(Debug, Serialize, Deserialize, Queryable, Clone, Default, SimpleObject)]
pub(crate) struct SlimCompany {
    /// Идентификатор компании на платформе
    pub(crate) uuid: Uuid,
    /// Сокращенное наименование
    pub(crate) shortname: String,
    /// Статус поставщика (в рамках платформы)
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

/// Аргументы для фильтрации и поиска по компаниям
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptCompaniesArg {
    /// Фильтр по Uuid компаний
    pub(crate) companies_uuids: Option<Vec<Uuid>>,
    /// Фильтр по владельцу компании
    pub(crate) user_uuid: Option<Uuid>,
    /// Фильтр по наличию компаний в избранном пользователя
    pub(crate) favorite: Option<bool>,
    /// Фильтр по статусу поставщика
    pub(crate) supplier: Option<bool>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
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
