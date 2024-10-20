use crate::errors::ServiceResult;
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::relate_ref::language::get_set_language;
use diesel::{sql_types, prelude::*};
use async_graphql::*;
use uuid::Uuid;

#[derive(QueryableByName)]
pub(super) struct ObjectUuid {
    #[diesel(sql_type = sql_types::Uuid)]
    uuid: Uuid
}

impl ObjectUuid {
    pub(super) fn get_uuids(objects: &[ObjectUuid]) -> Vec<Uuid> {
        let mut res = Vec::<Uuid>::new();
        for item in objects { res.push(item.uuid); }
        res
    }
}

#[derive(Debug)]
pub(crate) struct ExtraOptions {
    pub(crate) logged_user_uuid: Uuid,
    pub(crate) set_lang_id: i32,
}

impl ExtraOptions {
    pub(crate) fn from_cxt(cxt: &Context<'_>) -> ServiceResult<Self> {
        Ok(Self {
            logged_user_uuid: get_logged_user_uuid(cxt, true)?,
            set_lang_id: get_set_language(cxt),
        })
    }
}

/// Basic search attributes
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptSearchArg {
    pub(crate) search: String,
    #[graphql(default = false)]
    pub(crate) by_params: bool,
    #[graphql(default = false)]
    pub(crate) by_specs: bool,
    #[graphql(default = false)]
    pub(crate) by_keywords: bool,
    #[graphql(default = "")]
    pub(crate) order_by: String,
    #[graphql(default = false)]
    pub(crate) as_desc: bool,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) standard_uuid: Option<Uuid>,
    pub(crate) user_uuid: Option<Uuid>,
    #[graphql(default = false)]
    pub(crate) favorite: bool,
    #[graphql(default = 100)]
    pub(crate) limit: i32,
    #[graphql(default = 0)]
    pub(crate) offset: i32,
}