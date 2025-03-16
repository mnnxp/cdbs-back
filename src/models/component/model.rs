use crate::graphql::component_model::{IptComponentData, IptComponentsArg, IptComponentFilesArg};
use crate::models::component::util::get_root_component_uuid;
use crate::models::relate_ref::file::util::get_default_image;
use crate::schema::component_ref;
use chrono::{NaiveDateTime, Local};
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
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct ComponentsArg {
    pub(crate) filter_components_uuids: Vec<Uuid>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) standard_uuid: Option<Uuid>,
    pub(crate) user_uuid: Option<Uuid>,
    pub(crate) favorite: bool,
}

impl ComponentsArg {
    /// Returns a ComponentsArg with the given arguments
    pub(crate) fn by_arg(data: Option<IptComponentsArg>) -> Self {
        match data {
            Some(data) => Self {
                filter_components_uuids: data.components_uuids.unwrap_or_default(),
                company_uuid: data.company_uuid,
                standard_uuid: data.standard_uuid,
                user_uuid: data.user_uuid,
                favorite: data.favorite.unwrap_or_default(),
            },
            None => ComponentsArg::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ComponentFilesArg {
    pub(crate) component_uuid:  Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
}

impl From<IptComponentFilesArg> for ComponentFilesArg {
    fn from(data: IptComponentFilesArg) -> Self {
        let IptComponentFilesArg {
            component_uuid,
            files_uuids,
        } = data;

        Self {
            component_uuid,
            file_uuids: files_uuids.unwrap_or_default(),
        }
    }
}
