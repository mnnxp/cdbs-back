use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct License {
    pub id: i32,
    pub name: String,
    pub keyword: String,
    pub description: String,
    pub publication_at: NaiveDateTime,
}

#[Object]
impl License {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn keyword(&self) -> &String {
        &self.keyword
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn publication_at(&self) -> &NaiveDateTime {
        &self.publication_at
    }
}

#[derive(Debug, Insertable)]
#[table_name = "license_ref"]
pub struct InsertableLicense {
    pub name: String,
    pub keyword: String,
    pub description: String,
    pub publication_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct LicenseData {
    pub name: String,
    pub keyword: String,
    pub description: String,
    pub publication_at: NaiveDateTime,
}

// #[Object]
// impl LicenseData {
//     async fn name(&self) -> &String {
//         &self.name
//     }
//     async fn keyword(&self) -> &String {
//         &self.keyword
//     }
//     async fn description(&self) -> &String {
//         &self.description
//     }
//     async fn publication_at(&self) -> &NaiveDateTime {
//         &self.publication_at
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimLicense {
    pub id: i32,
    pub keyword: String,
}

#[Object]
impl SlimLicense {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn keyword(&self) -> &String {
        &self.keyword
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct LicenseToComponent {
    // pub id: i32,
    pub uuid_component: Uuid,
    pub id_license: i32,
}

#[Object]
impl LicenseToComponent {
    // async fn id(&self) -> &i32 {
    //     &self.id
    // }
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
    async fn id_license(&self) -> &i32 {
        &self.id_license
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptLicenseToComponentData {
    pub uuid_component: ID,
    pub id_license: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct LicenseToComponentData {
    pub uuid_component: Uuid,
    pub id_license: i32,
}

impl From<IptLicenseToComponentData> for LicenseToComponentData {
    fn from(ipt_data: IptLicenseToComponentData) -> Self {
        let IptLicenseToComponentData {
            uuid_component,
            id_license,
        } = ipt_data;
        LicenseToComponentData {
            uuid_component: Uuid::parse_str(&uuid_component.to_string()).unwrap(),
            id_license,
        }
    }
}

#[Object]
impl LicenseToComponentData {
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
    async fn id_license(&self) -> &i32 {
        &self.id_license
    }
}

#[derive(Debug, Insertable)]
#[table_name = "license_to_component"]
pub struct InsertableLicenseToComponent {
    pub uuid_component: Uuid,
    pub id_license: i32,
}

impl From<LicenseData> for InsertableLicense {
    fn from(data_license: LicenseData) -> Self {
        let LicenseData {
            name,
            keyword,
            description,
            publication_at,
            ..
        } = data_license;

        // let updated_at = chrono::Local::now().naive_local();

        Self {
            name,
            keyword,
            description,
            publication_at,
        }
    }
}

impl From<LicenseToComponentData> for InsertableLicenseToComponent {
    fn from(data_data_license_to_component: LicenseToComponentData) -> Self {
        let LicenseToComponentData {
            uuid_component,
            id_license,
            ..
        } = data_data_license_to_component;

        Self {
            uuid_component,
            id_license,
        }
    }
}
