use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct UserToken {
    pub uuid_user: Uuid,
    pub token: String,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "user_tokens_ref"]
pub struct InsertableUserToken {
    pub uuid_user: Uuid,
    pub token: String,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
}

#[Object]
impl UserToken {
    async fn uuid_user(&self) -> ID {
        self.uuid_user.into()
    }
    async fn token(&self) -> &String {
        &self.token
    }
    async fn start_at(&self) -> &NaiveDateTime {
        &self.start_at
    }
    async fn end_at(&self) -> &NaiveDateTime {
        &self.end_at
    }
}

impl From<UserToken> for InsertableUserToken {
    fn from(user_data: UserToken) -> Self {
        let UserToken {
            uuid_user,
            token,
            start_at,
            end_at,
            ..
        } = user_data;

        Self {
            uuid_user,
            token,
            start_at,
            end_at,
        }
    }
}
