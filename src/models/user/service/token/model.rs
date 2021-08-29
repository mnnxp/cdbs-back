use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct UserToken {
    pub uuid_user: Uuid,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expiration_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "user_token_ref"]
pub struct InsertableUserToken {
    pub uuid_user: Uuid,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expiration_at: NaiveDateTime,
}

#[Object]
impl UserToken {
    async fn uuid_user(&self) -> ID {
        self.uuid_user.into()
    }
    async fn token(&self) -> &String {
        &self.token
    }
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
    async fn expiration_at(&self) -> &NaiveDateTime {
        &self.expiration_at
    }
}

impl From<UserToken> for InsertableUserToken {
    fn from(user_data: UserToken) -> Self {
        let UserToken {
            uuid_user,
            token,
            created_at,
            expiration_at,
            ..
        } = user_data;

        Self {
            uuid_user,
            token,
            created_at,
            expiration_at,
        }
    }
}
