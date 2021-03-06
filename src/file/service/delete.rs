use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::user::model::{SlimUser, User};
use crate::user::util::verify;
use actix_web::web;
use diesel::prelude::*;

