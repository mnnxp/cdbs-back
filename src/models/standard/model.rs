use super::standard_status::model::StandardStatusTranslateList;
use crate::models::company::model::ShowCompanyShort;
use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::{
    type_access::model::TypeAccessTranslateList,
    file::model::{ShowFileRelatedData, DownloadFile},
    file::util::get_default_image,
    region::model::RegionTranslateList,
    spec::model::SpecTranslateList,
    keyword::model::Keyword,
};
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref ROOT_STANDARD_UUID : Uuid =
        Uuid::parse_str("303ec2aa-2066-42e3-93fb-de4fb9344bcb")
            .expect("Set root standard uuid failed!");
}

#[derive(Identifiable, Queryable, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = standard_ref)]
pub(crate) struct Standard {
    pub(crate) uuid: Uuid,
    pub(crate) parent_standard_uuid: Uuid,
    pub(crate) classifier: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) specified_tolerance: String,
    pub(crate) technical_committee: String,
    pub(crate) publication_at: NaiveDateTime,
    pub(crate) image_file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) standard_status_id: i32,
    pub(crate) region_id: i32,
    // pub(crate) is_delete: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Полная информация о стандарте (документе стандартизации) и связанные с ним данные
#[derive(Debug, SimpleObject)]
pub(crate) struct StandardAndRelatedData {
    /// Идентификатор стандарта на платформе
    pub(crate) uuid: Uuid,
    /// Идентификатор родительского стандарта
    pub(crate) parent_standard_uuid: Uuid,
    /// Классификация стандарта
    pub(crate) classifier: String,
    /// Наименование стандарта
    pub(crate) name: String,
    /// Описание стандарта
    pub(crate) description: String,
    /// Допуск стандарта
    pub(crate) specified_tolerance: String,
    /// Технический комитет (орган стандартизации)
    pub(crate) technical_committee: String,
    /// Дата публикации документа (стандарта)
    pub(crate) publication_at: NaiveDateTime,
    /// Данные для отображения основного изображения стандарта
    pub(crate) image_file: DownloadFile,
    /// Данные о загрузившем стандарт профиле
    pub(crate) owner_user: ShowUserShort,
    /// Данные о владеющей стандартом компании
    pub(crate) owner_company: ShowCompanyShort,
    /// Тип доступа к данным стандарта
    pub(crate) type_access: TypeAccessTranslateList,
    /// Актуальный статус стандарта (например, "в разработке")
    pub(crate) standard_status: StandardStatusTranslateList,
    /// Основной регион применения стандарта
    pub(crate) region: RegionTranslateList,
    /// Дата создания карточки стандарта
    pub(crate) created_at: NaiveDateTime,
    /// Дата обновления основных данных стандарта
    pub(crate) updated_at: NaiveDateTime,
    // Связанные со стандартом данные
    /// Файлы стандарта (документация и т.д.)
    pub(crate) standard_files: Vec<ShowFileRelatedData>,
    /// Каталоги в которые добавлен стандарт
    pub(crate) standard_specs: Vec<SpecTranslateList>,
    /// Ключевые слова (теги) стандарта
    pub(crate) standard_keywords: Vec<Keyword>,
    /// Количество добавивших стандарт в закладки
    pub(crate) subscribers: i32,
    /// Флаг наличия стандарта в закладках пользователя
    pub(crate) is_followed: bool,
}

/// Сокращенные данные о стандарте
#[derive(Debug, SimpleObject)]
pub(crate) struct ShowStandardShort {
    /// Идентификатор стандарта на платформе
    pub(crate) uuid: Uuid,
    /// Классификация стандарта
    pub(crate) classifier: String,
    /// Наименование стандарта
    pub(crate) name: String,
    /// Описание стандарта
    pub(crate) description: String,
    /// Допуск стандарта
    pub(crate) specified_tolerance: String,
    /// Дата публикации документа (стандарта)
    pub(crate) publication_at: NaiveDateTime,
    /// Данные для отображения основного изображения стандарта
    pub(crate) image_file: DownloadFile,
    /// Данные о владеющей стандартом компании
    pub(crate) owner_company: ShowCompanyShort,
    /// Актуальный статус стандарта (например, "в разработке")
    pub(crate) standard_status: StandardStatusTranslateList,
    /// Дата обновления основных данных стандарта
    pub(crate) updated_at: NaiveDateTime,
    /// Флаг наличия стандарта в закладках пользователя
    pub(crate) is_followed: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = standard_ref)]
pub(crate) struct InsertableStandard {
    uuid: Uuid,
    parent_standard_uuid: Uuid,
    classifier: String,
    name: String,
    description: String,
    specified_tolerance: String,
    technical_committee: String,
    publication_at: NaiveDateTime,
    image_file_uuid: Uuid,
    user_uuid: Uuid,
    company_uuid: Uuid,
    type_access_id: i32,
    standard_status_id: i32,
    region_id: i32,
    is_delete: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl InsertableStandard {
    /// Check parent standard uuid on nil
    pub(crate) fn parent_uuid_is_nil(&self) -> bool {
        self.parent_standard_uuid.is_nil()
    }

    /// Change parent uuid to base for insert new row
    pub(crate) fn parent_uuid_to_base(&mut self) {
        self.parent_standard_uuid = *ROOT_STANDARD_UUID;
    }

    /// Set image uuid (for set default image)
    pub(crate) fn set_image_uuid(&mut self) {
        self.image_file_uuid = get_default_image();
    }

    /// Set user uuid (for set logged user as owner)
    pub(crate) fn set_user_uuid(&mut self, user_uuid: &Uuid) {
        self.user_uuid = *user_uuid;
    }
}

/// Данные для регистрации нового стандарта на платформе
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptStandardData {
    /// Идентификатор родительского стандарта (опционально)
    pub(crate) parent_standard_uuid: Option<Uuid>,
    /// Классификация стандарта
    pub(crate) classifier: String,
    /// Наименование стандарта
    pub(crate) name: String,
    /// Описание стандарта
    pub(crate) description: String,
    /// Допуск стандарта
    pub(crate) specified_tolerance: String,
    /// Технический комитет (орган стандартизации)
    pub(crate) technical_committee: String,
    /// Дата публикации документа (стандарта)
    pub(crate) publication_at: NaiveDateTime,
    /// Идентификатор владеющей стандартом компании
    pub(crate) company_uuid: Uuid,
    /// Идентификатор типа доступа к данным стандарта
    pub(crate) type_access_id: i32,
    /// Идентификатор статуса состояния (готовности) стандарта
    pub(crate) standard_status_id: i32,
    /// Идентификатор региона применения (разработки) стандарта
    pub(crate) region_id: i32,
}

impl From<&IptStandardData> for InsertableStandard {
    fn from(ipt_data: &IptStandardData) -> Self {
        let IptStandardData {
            parent_standard_uuid,
            classifier,
            name,
            description,
            specified_tolerance,
            technical_committee,
            publication_at,
            company_uuid,
            type_access_id,
            standard_status_id,
            region_id,
        } = ipt_data;

        let parent_standard_uuid = match parent_standard_uuid {
            Some(parent_uuid) => *parent_uuid,
            None => Uuid::nil(),
        };

        Self {
            uuid: Uuid::new_v4(),
            parent_standard_uuid,
            classifier: classifier.clone(),
            name: name.clone(),
            description: description.clone(),
            specified_tolerance: specified_tolerance.clone(),
            technical_committee: technical_committee.clone(),
            publication_at: *publication_at,
            image_file_uuid: Uuid::nil(),
            user_uuid: Uuid::nil(),
            company_uuid: *company_uuid,
            type_access_id: *type_access_id,
            standard_status_id: *standard_status_id,
            region_id: *region_id,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

/// Данные для обновления карточки стандарта.
/// Обновление данных происходит только для заданных значений.
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateStandardData {
    /// Классификация стандарта
    pub(crate) classifier: Option<String>,
    /// Наименование стандарта
    pub(crate) name: Option<String>,
    /// Описание стандарта
    pub(crate) description: Option<String>,
    /// Допуск стандарта
    pub(crate) specified_tolerance: Option<String>,
    /// Технический комитет (орган стандартизации)
    pub(crate) technical_committee: Option<String>,
    /// Дата публикации документа (стандарта)
    pub(crate) publication_at: Option<NaiveDateTime>,
    /// Идентификатор владеющей стандартом компании
    pub(crate) company_uuid: Option<Uuid>,
    /// Идентификатор статуса состояния (готовности) стандарта
    pub(crate) standard_status_id: Option<i32>,
    /// Идентификатор региона применения (разработки) стандарта
    pub(crate) region_id: Option<i32>,
}

/// Аргументы для фильтрации и поиска по стандартам
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardsArg {
    /// Фильтр по Uuid стандартов
    pub(crate) standards_uuids:  Option<Vec<Uuid>>,
    /// Фильтр по компании-владельцу стандарта
    pub(crate) company_uuid: Option<Uuid>,
    /// Фильтр по наличию стандарта в избранном пользователя
    pub(crate) favorite: Option<bool>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct StandardsArg {
    pub(crate) filter_standards_uuids: Vec<Uuid>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) favorite: bool,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for StandardsArg {
    fn default() -> Self {
        Self {
            filter_standards_uuids: Vec::new(),
            company_uuid: None,
            favorite: false,
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptStandardsArg> for StandardsArg {
    fn from(data: IptStandardsArg) -> Self {
        let IptStandardsArg {
            standards_uuids,
            company_uuid,
            favorite,
            limit,
            offset,
        } = data;

        Self {
            filter_standards_uuids: standards_uuids.unwrap_or_default(),
            company_uuid,
            favorite: favorite.unwrap_or(false),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}

/// Аргументы для фильтрации и поиска по файлам стандарта
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardFilesArg {
    /// Фильтр по Uuid стандарта
    pub(crate) standard_uuid:  Uuid,
    /// Фильтр по Uuid файлам стандарта
    pub(crate) files_uuids: Option<Vec<Uuid>>,
    /// Ограничение выборки данных (максимальное кол-во записей)
    pub(crate) limit: Option<i32>,
    /// Кол-во пропущенных записей в начале (смещение)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct StandardFilesArg {
    pub(crate) standard_uuid:  Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl From<IptStandardFilesArg> for StandardFilesArg {
    fn from(data: IptStandardFilesArg) -> Self {
        Self {
            standard_uuid: data.standard_uuid,
            file_uuids: data.files_uuids.unwrap_or_default(),
            limit: data.limit.unwrap_or(100),
            offset: data.offset.unwrap_or(0),
        }
    }
}
