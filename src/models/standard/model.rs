use crate::graphql::standard_model::{IptStandardData, IptStandardsArg, IptStandardFilesArg};
use crate::models::relate_ref::file::util::get_default_image;
use crate::schema::standard_ref;
use chrono::{Local, NaiveDateTime};
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
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) publication_at: NaiveDateTime,
    pub(crate) image_file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) standard_status_id: i32,
    // pub(crate) is_delete: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = standard_ref)]
pub(crate) struct InsertableStandard {
    uuid: Uuid,
    parent_standard_uuid: Uuid,
    name: String,
    description: String,
    publication_at: NaiveDateTime,
    image_file_uuid: Uuid,
    user_uuid: Uuid,
    company_uuid: Uuid,
    type_access_id: i32,
    standard_status_id: i32,
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

impl From<&IptStandardData> for InsertableStandard {
    fn from(ipt_data: &IptStandardData) -> Self {
        let IptStandardData {
            parent_standard_uuid,
            name,
            description,
            publication_at,
            company_uuid,
            type_access_id,
            standard_status_id,
            ..
        } = ipt_data;

        let parent_standard_uuid = match parent_standard_uuid {
            Some(parent_uuid) => *parent_uuid,
            None => Uuid::nil(),
        };

        Self {
            uuid: Uuid::new_v4(),
            parent_standard_uuid,
            name: name.clone(),
            description: description.clone(),
            publication_at: *publication_at,
            image_file_uuid: Uuid::nil(),
            user_uuid: Uuid::nil(),
            company_uuid: *company_uuid,
            type_access_id: *type_access_id,
            standard_status_id: *standard_status_id,
            is_delete: false,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct StandardsArg {
    pub(crate) filter_standards_uuids: Vec<Uuid>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) favorite: bool,
}

impl StandardsArg {
    /// Returns a StandardsArg with the given arguments
    pub(crate) fn by_arg(data: Option<IptStandardsArg>) -> Self {
        match data {
            Some(data) => Self {
                filter_standards_uuids: data.standards_uuids.unwrap_or_default(),
                company_uuid: data.company_uuid,
                favorite: data.favorite.unwrap_or_default(),
            },
            None => StandardsArg::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct StandardFilesArg {
    pub(crate) standard_uuid: Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
}

impl From<IptStandardFilesArg> for StandardFilesArg {
    fn from(data: IptStandardFilesArg) -> Self {
        Self {
            standard_uuid: data.standard_uuid,
            file_uuids: data.files_uuids.unwrap_or_default(),
        }
    }
}
