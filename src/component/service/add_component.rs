use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::user::model::{InsertableUser, SlimUser, User, UserData};
use actix_web::web;
use diesel::prelude::*;
