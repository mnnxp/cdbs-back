use crate::database::Pool;
use crate::errors::ServiceError;
use crate::user::model::{LoggedUser, SlimUser, UserData};
use crate::user::service as user;
use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};
