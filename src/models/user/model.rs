use super::access::hash::{make_hash_salt, make_salt};
use super::certificate::model::UserCertificateAndFile;
use crate::models::relate_ref::{
    file::model::DownloadFile,
    file::util::get_default_image,
    region::model::RegionTranslateList,
    program::model::Program,
    type_access::model::TypeAccessTranslateList,
};
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub(crate) struct User {
    uuid: Uuid,
    // email: String,
    psw_hash: Vec<u8>,
    psw_salt: Vec<u8>,
    // firstname: String,
    // lastname: String,
    // secondname: String,
    username: String,
    // phone: String,
    // description: String,
    // address: String,
    // position: String,
    // time_zone: String,
    // image_file_uuid: Uuid,
    // region_id: i32,
    program_id: i32,
    // type_access_id: i32,
    // is_email_verified: bool,
    // is_enabled: bool,
    // is_delete: bool,
    // created_at: NaiveDateTime,
    // updated_at: NaiveDateTime,
}

impl User {
    /// Gets password hash
    pub(super) fn get_psw_hash(&self) -> &[u8] {
        &self.psw_hash
    }

    /// Gets password salt
    pub(super) fn get_psw_salt(&self) -> &[u8] {
        &self.psw_salt
    }
}

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = user_ref)]
pub(crate) struct UserQuery {
    pub(crate) uuid: Uuid,
    pub(crate) email: String,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
    pub(crate) secondname: String,
    pub(crate) username: String,
    pub(crate) phone: String,
    pub(crate) description: String,
    pub(crate) address: String,
    pub(crate) position: String,
    pub(crate) time_zone: String,
    pub(crate) image_file_uuid: Uuid,
    pub(crate) region_id: i32,
    pub(crate) program_id: i32,
    pub(crate) type_access_id: i32,
    pub(crate) is_email_verified: bool,
    // pub(crate) is_enabled: bool,
    // pub(crate) is_delete: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Полная информация о собственном профиле пользователя
#[derive(Debug, SimpleObject)]
pub(crate) struct UserAndRelatedData {
    /// Идентификатор пользователя на платформе
    pub(crate) uuid: Uuid,
    /// Эл.почта пользователя
    pub(crate) email: String,
    /// Имя
    pub(crate) firstname: String,
    /// Фамилия
    pub(crate) lastname: String,
    /// Отчество
    pub(crate) secondname: String,
    /// Имя пользователя (никнейм)
    pub(crate) username: String,
    /// Номер телефона пользователя
    pub(crate) phone: String,
    /// Описание пользователя
    pub(crate) description: String,
    /// Адрес пользователя
    pub(crate) address: String,
    /// Позиция пользователя (должность/специализация)
    pub(crate) position: String, // <-- todo!(create a separate table with translation)
    /// Временная зона пользователя
    pub(crate) time_zone: String,
    /// Данные для отображения основного изображения пользователя (аватарки)
    pub(crate) image_file: DownloadFile,
    /// Регион пользователя
    pub(crate) region: RegionTranslateList,
    /// Основной программный инструмент пользователя (САПР, программа)
    pub(crate) program: Program,
    /// Тип доступа к данным пользователя
    pub(crate) type_access: TypeAccessTranslateList,
    /// Флаг результата подтверждения эл. почты
    pub(crate) is_email_verified: bool,
    /// Дата создания профиля пользователя
    pub(crate) created_at: NaiveDateTime,
    /// Дата обновления основных данных пользователя
    pub(crate) updated_at: NaiveDateTime,
    // Связанные данные
    /// Список сертификатов и грамот пользователя
    pub(crate) certificates: Vec<UserCertificateAndFile>,
    /// Количество добавивших пользователя в закладки
    pub(crate) subscribers: i32,
    // for a quick request just count objects have user
    /// Кол-во созданных пользователем компаний
    pub(crate) companies_count: i32,
    /// Кол-во созданных пользователем компонентов
    pub(crate) components_count: i32,
    /// Кол-во созданных пользователем стандартов
    pub(crate) standards_count: i32,
    // for a quick request just count the subscribers
    /// Кол-во компаний в закладках пользователя
    pub(crate) fav_companies_count: i32,
    /// Кол-во компонентов в закладках пользователя
    pub(crate) fav_components_count: i32,
    /// Кол-во стандартов в закладках пользователя
    pub(crate) fav_standards_count: i32,
    /// Кол-во пользователей в закладках пользователя
    pub(crate) fav_users_count: i32,
}

/// Полная информация о профиле пользователя
#[derive(Debug, SimpleObject)]
pub(crate) struct ShowUserAndRelatedData {
    /// Идентификатор пользователя на платформе
    pub(crate) uuid: Uuid,
    /// Имя
    pub(crate) firstname: String,
    /// Фамилия
    pub(crate) lastname: String,
    /// Отчество
    pub(crate) secondname: String,
    /// Имя пользователя (никнейм)
    pub(crate) username: String,
    /// Описание пользователя
    pub(crate) description: String,
    /// Позиция пользователя (должность/специализация)
    pub(crate) position: String, // <-- todo!(create a separate table with translation)
    /// Данные для отображения основного изображения пользователя (аватарки)
    pub(crate) image_file: DownloadFile,
    /// Регион пользователя
    pub(crate) region: RegionTranslateList,
    /// Основной программный инструмент пользователя (САПР, программа)
    pub(crate) program: Program,
    /// Дата создания профиля пользователя
    pub(crate) created_at: NaiveDateTime,
    /// Дата обновления основных данных пользователя
    pub(crate) updated_at: NaiveDateTime,
    // Связанные данные
    /// Список сертификатов и грамот пользователя
    pub(crate) certificates: Vec<UserCertificateAndFile>,
    /// Количество добавивших пользователя в закладки
    pub(crate) subscribers: i32,
    /// Флаг наличия пользователя в закладках пользователя (зрителя)
    pub(crate) is_followed: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_ref)]
pub(crate) struct InsertableUser {
    uuid: Uuid,
    email: String,
    psw_hash: Vec<u8>,
    psw_salt: Vec<u8>,
    firstname: String,
    lastname: String,
    secondname: String,
    username: String,
    phone: String,
    description: String,
    address: String,
    position: String,
    time_zone: String,
    image_file_uuid: Uuid,
    region_id: i32,
    program_id: i32,
    type_access_id: i32,
    is_email_verified: bool,
    is_enabled: bool,
    is_delete: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

/// Данные для добавления нового пользователя
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUserData {
    /// Эл.почта пользователя
    pub(crate) email: String,
    /// Имя пользователя (никнейм)
    pub(crate) username: String,
    /// Пароль
    pub(crate) password: String,
    /// Имя (опционально)
    pub(crate) firstname: Option<String>,
    /// Фамилия (опционально)
    pub(crate) lastname: Option<String>,
    /// Отчество (опционально)
    pub(crate) secondname: Option<String>,
    /// Номер телефона пользователя (опционально)
    pub(crate) phone: Option<String>,
    /// Описание пользователя (опционально)
    pub(crate) description: Option<String>,
    /// Адрес пользователя (опционально)
    pub(crate) address: Option<String>,
    /// Позиция пользователя (должность/специализация) (опционально)
    pub(crate) position: Option<String>,
    /// Временная зона пользователя (опционально)
    pub(crate) time_zone: Option<String>,
    /// Идентификатор региона пользователя (опционально)
    pub(crate) region_id: Option<i32>,
    /// Идентификатор основного софта пользователя (например, какого-нибудь САПР) (опционально)
    pub(crate) program_id: Option<i32>,
    /// Идентификатор типа доступа к данным пользователя (опционально)
    pub(crate) type_access_id: Option<i32>,
}

impl From<&IptUserData> for InsertableUser {
    fn from(ipt_data: &IptUserData) -> Self {
        let IptUserData {
            email,
            username,
            password,
            firstname,
            lastname,
            secondname,
            phone,
            description,
            address,
            position,
            time_zone,
            region_id,
            program_id,
            type_access_id,
            ..
        } = ipt_data;

        let psw_salt = make_salt();
        let psw_hash = make_hash_salt(
            password.as_bytes(),
            &psw_salt,
        );

        // set default data
        let firstname = match firstname {
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let lastname = match lastname{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let secondname = match secondname{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let phone = match phone{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let description = match description{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let address = match address{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let position = match position{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let time_zone = match time_zone{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let region_id = region_id.unwrap_or(1);
        let program_id = program_id.unwrap_or(1);
        let type_access_id = type_access_id.unwrap_or(3);

        Self {
            uuid: Uuid::new_v4(),
            email: email.to_string(),
            psw_hash,
            psw_salt: psw_salt.to_vec(),
            firstname,
            lastname,
            secondname,
            username: username.to_string(),
            phone,
            description,
            address,
            position,
            time_zone,
            // default favicon image
            image_file_uuid: get_default_image(),
            region_id,
            program_id,
            type_access_id,
            is_email_verified: false,
            is_enabled: true,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

/// Минимальные данные о пользователе
#[derive(Debug, Serialize, Deserialize, Queryable, Clone, SimpleObject)]
pub(crate) struct SlimUser {
    /// Идентификатор пользователя на платформе
    pub(crate) uuid: Uuid,
    /// Имя пользователя (никнейм)
    pub(crate) username: String,
    /// Идентификатор основного программного инструмента пользователя
    pub(crate) program_id: i32,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            uuid,
            username,
            program_id,
            ..
        } = user;

        Self {
            uuid,
            username,
            program_id,
        }
    }
}

#[derive(Identifiable, Serialize, Queryable, Clone, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = user_ref)]
pub(crate) struct UserShort {
    pub(crate) uuid: Uuid,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
    pub(crate) username: String,
    pub(crate) image_file_uuid: Uuid,
}

#[derive(Clone, SimpleObject, Debug)]
pub(crate) struct ShowUserShort {
    pub(crate) uuid: Uuid,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
    pub(crate) username: String,
    pub(crate) image_file: DownloadFile,
}

impl ShowUserShort {
    /// Create struct with UserShort data, DownloadFile data set default
    pub(crate) fn new(data: &UserShort) -> Self {
        Self{
            uuid: data.uuid,
            firstname: data.firstname.clone(),
            lastname: data.lastname.clone(),
            username: data.username.clone(),
            image_file: Default::default(),
        }
    }

    /// Change image_file data
    pub(crate) fn put_image_file(&mut self, image_file: DownloadFile) {
        self.image_file = image_file;
    }
}

/// Данные для обновления профиля пользователя.
/// Обновление данных происходит только для заданных значений.
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateUserData {
    /// Эл.почта пользователя
    pub(crate) email: Option<String>,
    /// Имя
    pub(crate) firstname: Option<String>,
    /// Фамилия
    pub(crate) lastname: Option<String>,
    /// Отчество
    pub(crate) secondname: Option<String>,
    /// Имя пользователя (никнейм)
    pub(crate) username: Option<String>,
    /// Номер телефона пользователя
    pub(crate) phone: Option<String>,
    /// Описание пользователя
    pub(crate) description: Option<String>,
    /// Адрес пользователя
    pub(crate) address: Option<String>,
    /// Позиция пользователя (должность/специализация)
    pub(crate) position: Option<String>,
    /// Временная зона пользователя
    pub(crate) time_zone: Option<String>,
    /// Идентификатор региона пользователя
    pub(crate) region_id: Option<i32>,
    /// Идентификатор основного софта пользователя (например, какого-нибудь САПР)
    pub(crate) program_id: Option<i32>,
}

/// Аргументы для фильтрации и поиска по компаниям
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptUsersArg {
    /// Фильтр Uuid пользователей
    pub(crate) users_uuids:  Option<Vec<Uuid>>,
    /// Фильтр по подписчикам активного пользователя
    pub(crate) subscribers: Option<bool>,
    /// Фильтр по наличию пользователей в избранном активного пользователя
    pub(crate) favorite: Option<bool>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct UsersArg {
    pub(crate) filter_users_uuids: Vec<Uuid>,
    pub(crate) subscribers: bool,
    pub(crate) favorite: bool,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for UsersArg {
    fn default() -> Self {
        Self {
            filter_users_uuids: Vec::new(),
            subscribers: false,
            favorite: false,
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptUsersArg> for UsersArg {
    fn from(data: IptUsersArg) -> Self {
        let IptUsersArg {
            users_uuids,
            subscribers,
            favorite,
            limit,
            offset,
        } = data;

        Self {
            filter_users_uuids: users_uuids.unwrap_or_default(),
            subscribers: subscribers.unwrap_or(false),
            favorite: favorite.unwrap_or(false),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}

/// Получение полных данных профиля пользователя
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptGetUserArg {
    /// Используется для получения данных по Uuid пользователя
    pub(crate) user_uuid: Option<Uuid>,
    /// Используется для получения данных по имени пользователя (никнейму)
    pub(crate) username: Option<String>,
}
