use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct UserToken {
    pub user_uuid: Uuid,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expiration_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "user_token_ref"]
pub struct InsertableUserToken {
    pub user_uuid: Uuid,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expiration_at: NaiveDateTime,
}

#[Object]
impl UserToken {
    async fn user_uuid(&self) -> ID {
        self.user_uuid.into()
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
            user_uuid,
            token,
            created_at,
            expiration_at,
            ..
        } = user_data;

        Self {
            user_uuid,
            token,
            created_at,
            expiration_at,
        }
    }
}
