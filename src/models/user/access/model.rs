use crate::jwt::model::Claims;
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable, SimpleObject)]
pub struct UserToken {
    pub user_uuid: Uuid,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expiration_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "user_token_ref"]
pub(super) struct InsertableUserToken {
    user_uuid: Uuid,
    token: String,
    created_at: NaiveDateTime,
    expiration_at: NaiveDateTime,
}

impl InsertableUserToken {
    /// Create based on data without token
    pub(super) fn new(user_uuid: &Uuid, jwt: &Claims) -> Self {
        Self{
            user_uuid: *user_uuid,
            token: String::new(),
            created_at: NaiveDateTime::from_timestamp(jwt.iat, 0),
            expiration_at: NaiveDateTime::from_timestamp(jwt.exp, 0),
        }
    }

    /// Change token data
    pub(super) fn put_token(&mut self, token: &str) {
        self.token = token.to_string();
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
