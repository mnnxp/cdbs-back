use crate::schema::*;
use crate::models::user::model::ShowUser;
use crate::models::relate_ref::file::model::ShowFile;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Certificate for User
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(uuid_file, uuid_user)]
#[belongs_to(ShowFile, foreign_key = "uuid_file")]
#[belongs_to(ShowUser, foreign_key = "uuid_user")]
#[table_name = "user_certificate_ref"]
pub struct UserCertificate {
    pub uuid_file: Uuid,
    pub uuid_user: Uuid,
    pub description: String,
}

#[Object]
impl UserCertificate {
    async fn uuid_file(&self) -> ID {
        self.uuid_file.into()
    }
    async fn uuid_user(&self) -> ID {
        self.uuid_user.into()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "user_certificate_ref"]
pub struct InsertableUserCertificate {
    pub uuid_file: Uuid,
    pub uuid_user: Uuid,
    pub description: String,
}

impl From<UserCertificate> for InsertableUserCertificate {
    fn from(ipt_data: UserCertificate) -> Self {
        let UserCertificate {
            uuid_file,
            uuid_user,
            description,
            ..
        } = ipt_data;

        Self {
            uuid_file,
            uuid_user,
            description,
        }
    }
}
