use crate::errors::ServiceResult;
use crate::graphql::handler::extract_client_domain;
use crate::models::relate_ref::language::get_set_language;
use crate::auth::token::logged::{default_user_uuid, get_logged_user_uuid};
use crate::models::user::model::SlimUser;
use async_graphql::*;
use diesel::{prelude::*, sql_types};
use uuid::Uuid;

#[derive(QueryableByName)]
pub(crate) struct ObjectUuid {
    #[diesel(sql_type = sql_types::Uuid)]
    uuid: Uuid,
}

impl ObjectUuid {
    pub(crate) fn get_uuids(objects: &[ObjectUuid]) -> Vec<Uuid> {
        let mut res = Vec::<Uuid>::new();
        for item in objects {
            res.push(item.uuid);
        }
        res
    }
}

#[derive(Debug, QueryableByName)]
pub(crate) struct ObjectI64 {
    #[diesel(sql_type = sql_types::BigInt)]
    pub(crate) count: i64,
}

#[derive(Debug)]
pub(crate) struct ExtraOptions {
    pub(crate) logged_user_uuid: Uuid,
    pub(crate) set_lang_id: i32,
    pub(crate) domain: String,
    pub(crate) no_entry: bool,
}

impl ExtraOptions {
    /// Returns the structure with logged user uuid and set language.
    /// If token validation fails and no_entry is true, will be made to retrieve the default user UUID.
    /// If the default user UUID could not be obtained, the first error received during token validation will be returned.
    pub(crate) fn from_cxt(cxt: &Context<'_>, no_entry: bool) -> ServiceResult<Self> {
        let set_lang_id = get_set_language(cxt);
        let domain = extract_client_domain(cxt);
        match get_logged_user_uuid(cxt, true) {
            Ok(logged_user_uuid) => Ok(Self {
                logged_user_uuid,
                set_lang_id,
                domain,
                no_entry: false,
            }),
            Err(err) => {
                if let (Ok(logged_user_uuid), true) = (default_user_uuid(cxt), no_entry) {
                    // default user uuid and set language
                    return Ok(Self {
                        logged_user_uuid,
                        set_lang_id,
                        domain,
                        no_entry: true,
                    });
                }
                // error message
                Err(err)
            }
        }
    }

    pub(crate) fn by_slim_user(cxt: &Context<'_>, slim_user: &SlimUser) -> Self {
        Self {
            logged_user_uuid: slim_user.uuid,
            set_lang_id: get_set_language(cxt),
            domain: extract_client_domain(cxt),
            no_entry: false,
        }
    }
}

/// Basic search attributes
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptSearchArg {
    pub(crate) search: String,
    #[graphql(default = true)]
    pub(crate) by_params: bool,
    #[graphql(default = true)]
    pub(crate) by_specs: bool,
    #[graphql(default = true)]
    pub(crate) by_keywords: bool,
    #[graphql(default = true)]
    pub(crate) by_modifications: bool,
    #[graphql(default = true)]
    pub(crate) by_modification_params: bool,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) standard_uuid: Option<Uuid>,
    pub(crate) service_uuid: Option<Uuid>,
    pub(crate) user_uuid: Option<Uuid>,
    pub(crate) spec_id: Option<i32>,
    #[graphql(default = false)]
    pub(crate) favorite: bool,
}
