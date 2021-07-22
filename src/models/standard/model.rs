use crate::schema::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct Standard {
    pub id: i32,
    pub uuid: Uuid,
    pub uuid_standard_parent: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub uuid_image_file: Uuid,
    pub uuid_user: Uuid,
    pub uuid_company: Uuid,
    pub id_type_access: i32,
    pub id_standard_status: i32,
    pub id_region: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ShowStandard {
    pub uuid: Uuid,
    pub uuid_standard_parent: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub uuid_image_file: Uuid,
    pub uuid_user: Uuid,
    pub uuid_company: Uuid,
    pub id_type_access: i32,
    pub id_standard_status: i32,
    pub id_region: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "standard_ref"]
pub struct InsertableStandard {
    pub uuid: Uuid,
    pub uuid_standard_parent: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub uuid_image_file: Uuid,
    pub uuid_user: Uuid,
    pub uuid_company: Uuid,
    pub id_type_access: i32,
    pub id_standard_status: i32,
    pub id_region: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct StandardData {
    pub uuid_standard_parent: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub uuid_image_file: Uuid,
    pub uuid_user: Uuid,
    pub uuid_company: Uuid,
    pub id_type_access: i32,
    pub id_standard_status: i32,
    pub id_region: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimStandard {
    pub uuid: Uuid,
    pub classifier: String,
    pub name: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub id_standard_status: i32,
}

impl From<StandardData> for InsertableStandard {
    fn from(company_data: StandardData) -> Self {
        let StandardData {
            uuid_standard_parent,
            classifier,
            name,
            description,
            specified_tolerance,
            technical_committee,
            publication_at,
            uuid_image_file,
            uuid_user,
            uuid_company,
            id_type_access,
            id_standard_status,
            id_region,
            ..
        } = company_data;

        Self {
            uuid: Uuid::new_v4(),
            uuid_standard_parent,
            classifier,
            name,
            description,
            specified_tolerance,
            technical_committee,
            publication_at,
            uuid_image_file,
            uuid_user,
            uuid_company,
            id_type_access,
            id_standard_status,
            id_region,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

impl From<Standard> for SlimStandard {
    fn from(company: Standard) -> Self {
        let Standard {
            uuid,
            classifier,
            name,
            specified_tolerance,
            technical_committee,
            publication_at,
            id_standard_status,
            ..
        } = company;

        Self {
            uuid,
            classifier,
            name,
            specified_tolerance,
            technical_committee,
            publication_at,
            id_standard_status,
        }
    }
}
