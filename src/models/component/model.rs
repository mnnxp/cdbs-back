use crate::models::component::{
    component_type::model::ComponentTypeTranslateList,
    actual_status::model::ActualStatusTranslateList,
    param::model::ComponentParamWithTranslation,
    supplier::model::ComponentSupplierRelatedData,
    component_modification::model::ComponentModificationAndRelatedData,
    util::get_root_component_uuid,
};
use crate::models::user::model::ShowUserShort;
use crate::models::standard::model::ShowStandardShort;
use crate::models::relate_ref::{
    type_access::model::TypeAccessTranslateList,
    license::model::License,
    keyword::model::Keyword,
    file::model::{ShowFileRelatedData, DownloadFile},
    file::util::get_default_image,
    spec::model::SpecTranslateList,
};
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = component_ref)]
pub(crate) struct Component {
    pub(crate) uuid: Uuid,
    pub(crate) parent_component_uuid: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) image_file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) component_type_id: i32,
    pub(crate) actual_status_id: i32,
    pub(crate) is_base: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Полная информация о компоненте (части) и связанные с ним данные
#[derive(Debug, SimpleObject)]
pub(crate) struct ComponentAndRelatedData {
    /// Идентификатор компонента на платформе
    pub(crate) uuid: Uuid,
    /// Идентификатор родительского компонента
    pub(crate) parent_component_uuid: Uuid,
    /// Наименование компонента
    pub(crate) name: String,
    /// Описание компонента
    pub(crate) description: String,
    /// Данные для отображения основного вида компонента (части)
    pub(crate) image_file: DownloadFile,
    /// Данные о профиле владеющем компонентом
    pub(crate) owner_user: ShowUserShort,
    /// Тип доступа к данным компонента
    pub(crate) type_access: TypeAccessTranslateList,
    /// Тип компонента (например, "стандартный")
    pub(crate) component_type: ComponentTypeTranslateList,
    /// Актуальный статус компонента (например, "в разработке")
    pub(crate) actual_status: ActualStatusTranslateList,
    /// Для базовых компонентов возможна связь с множеством производителей/поставщиков
    pub(crate) is_base: bool,
    /// Количество добавивших компонент в закладки
    pub(crate) subscribers: i32,
    /// Флаг наличия компонента в закладках пользователя
    pub(crate) is_followed: bool,
    /// Дата создания профиля компонента
    pub(crate) created_at: NaiveDateTime,
    /// Дата обновления основных данных компонента
    pub(crate) updated_at: NaiveDateTime,
    // Связанные с компонентом данные
    /// Лицензии распространения данных компонента
    pub(crate) licenses: Vec<License>,
    /// Список параметров компонента
    pub(crate) component_params: Vec<ComponentParamWithTranslation>,
    /// Связанные с компонентом файлы
    pub(crate) files: Vec<ShowFileRelatedData>,
    /// Каталоги в которые добавлен компонент
    pub(crate) component_specs: Vec<SpecTranslateList>,
    /// Ключевые слова (теги) компонента
    pub(crate) component_keywords: Vec<Keyword>,
    /// Модификации компонента и связанные с ними данные (такие как наборы файлов для САПР)
    pub(crate) component_modifications: Vec<ComponentModificationAndRelatedData>,
    /// Компания-производитель или поставщики компонента (если is_base истина)
    pub(crate) component_suppliers: Vec<ComponentSupplierRelatedData>,
    /// Список связанных с компонентом документов стандартизации
    pub(crate) component_standards: Vec<ShowStandardShort>,
}

/// Сокращенные данные о компоненте
#[derive(Debug, SimpleObject)]
pub(crate) struct ShowComponentShort {
    /// Идентификатор компонента на платформе
    pub(crate) uuid: Uuid,
    /// Наименование компонента
    pub(crate) name: String,
    /// Описание компонента
    pub(crate) description: String,
    /// Данные для отображения основного вида компонента (части)
    pub(crate) image_file: DownloadFile,
    /// Данные о профиле владеющем компонентом
    pub(crate) owner_user: ShowUserShort,
    /// Тип доступа к данным компонента
    pub(crate) type_access: TypeAccessTranslateList,
    /// Тип компонента (например, "стандартный")
    pub(crate) component_type: ComponentTypeTranslateList,
    /// Актуальный статус компонента (например, "в разработке")
    pub(crate) actual_status: ActualStatusTranslateList,
    /// Для базовых компонентов возможна связь с множеством производителей/поставщиков
    pub(crate) is_base: bool,
    /// Флаг наличия компонента в закладках пользователя
    pub(crate) is_followed: bool,
    /// Дата обновления основных данных компонента
    pub(crate) updated_at: NaiveDateTime,
    /// Лицензии распространения данных компонента
    pub(crate) licenses: Vec<License>,
    /// Связанные с компонентом файлы (изображения)
    pub(crate) files: Vec<DownloadFile>,
    /// Компания-производитель или поставщики компонента (если is_base истина)
    pub(crate) component_suppliers: Vec<ComponentSupplierRelatedData>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = component_ref)]
pub(crate) struct InsertableComponent {
    uuid: Uuid,
    parent_component_uuid: Uuid,
    name: String,
    description: String,
    image_file_uuid: Uuid,
    user_uuid: Uuid,
    type_access_id: i32,
    component_type_id: i32,
    actual_status_id: i32,
    is_base: bool,
    is_delete: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl InsertableComponent {
    /// Check parent component uuid on nil
    pub(crate) fn parent_uuid_is_nil(&self) -> bool {
        self.parent_component_uuid.is_nil()
    }

    /// Change parent uuid to base for insert new row
    pub(crate) fn parent_uuid_to_base(&mut self) {
        self.parent_component_uuid = get_root_component_uuid();
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

#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptComponentData {
    pub(crate) parent_component_uuid: Option<Uuid>,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) type_access_id: i32,
    pub(crate) component_type_id: i32,
    pub(crate) actual_status_id: i32,
    pub(crate) is_base: bool,
}

impl From<&IptComponentData> for InsertableComponent {
    fn from(ipt_data: &IptComponentData) -> Self {
        let IptComponentData {
            parent_component_uuid,
            name,
            description,
            type_access_id,
            component_type_id,
            actual_status_id,
            is_base,
            ..
        } = ipt_data;

        let parent_component_uuid = match parent_component_uuid {
            Some(parent_uuid) => *parent_uuid,
            None => Uuid::nil(),
        };

        Self {
            uuid: Uuid::new_v4(),
            parent_component_uuid,
            name: name.clone(),
            description: description.clone(),
            image_file_uuid: Uuid::nil(),
            user_uuid: Uuid::nil(),
            type_access_id: *type_access_id,
            component_type_id: *component_type_id,
            actual_status_id: *actual_status_id,
            is_base: *is_base,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptUpdateComponentData {
    pub(crate) parent_component_uuid: Option<Uuid>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) component_type_id: Option<i32>,
    pub(crate) actual_status_id: Option<i32>,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentsArg {
    pub(crate) components_uuids:  Option<Vec<Uuid>>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) standard_uuid: Option<Uuid>,
    pub(crate) user_uuid: Option<Uuid>,
    pub(crate) favorite: Option<bool>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ComponentsArg {
    pub(crate) filter_components_uuids: Vec<Uuid>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) standard_uuid: Option<Uuid>,
    pub(crate) user_uuid: Option<Uuid>,
    pub(crate) favorite: bool,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for ComponentsArg {
    fn default() -> Self {
        Self {
            filter_components_uuids: Vec::new(),
            company_uuid: None,
            standard_uuid: None,
            user_uuid: None,
            favorite: false,
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptComponentsArg> for ComponentsArg {
    fn from(data: IptComponentsArg) -> Self {
        let IptComponentsArg {
            components_uuids,
            company_uuid,
            standard_uuid,
            user_uuid,
            favorite,
            limit,
            offset,
        } = data;

        Self {
            filter_components_uuids: components_uuids.unwrap_or_default(),
            company_uuid,
            standard_uuid,
            user_uuid,
            favorite: favorite.unwrap_or(false),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentFilesArg {
    pub(crate) component_uuid:  Uuid,
    pub(crate) files_uuids: Option<Vec<Uuid>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ComponentFilesArg {
    pub(crate) component_uuid:  Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl From<IptComponentFilesArg> for ComponentFilesArg {
    fn from(data: IptComponentFilesArg) -> Self {
        let IptComponentFilesArg {
            component_uuid,
            files_uuids,
            limit,
            offset,
        } = data;

        Self {
            component_uuid,
            file_uuids: files_uuids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
