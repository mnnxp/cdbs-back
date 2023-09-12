use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::relate_ref::{
    type_access,
    type_access::model::{
        IptTypeAccessTranslateListData, TypeAccessTranslateList,
        IptTypeAccessArg, TypeAccessArg
    },
    language::get_set_language,
};
use crate::models::user::access::logged::check_authorized;
use async_graphql::{self, Context, Object};

#[derive(Default)]
pub struct TypeAccessQuery;
#[derive(Default)]
pub struct TypeAccessMutation;

#[Object]
impl TypeAccessQuery {
    /// Returns types access by IDs.
    /// If a filter for types access is not specified, then all existing ones are aggregated.
    async fn types_access(
        &self,
        cxt: &Context<'_>,
        args: Option<IptTypeAccessArg>,
    ) -> ServiceResult<Vec<TypeAccessTranslateList>> {
        use type_access::service::list::get_type_access;

        let arguments = match args {
            Some(x) => TypeAccessArg::from(x),
            None => TypeAccessArg::default(),
        };

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_type_access(&arguments, &get_set_language(cxt), conn)
    }
}

#[Object]
impl TypeAccessMutation {
    /// Adds a new type access.
    /// Returns an error with the type access ID if it already exists.
    async fn register_type_access(
        &self,
        cxt: &Context<'_>,
        args: IptTypeAccessTranslateListData,
    ) -> ServiceResult<TypeAccessTranslateList> {
        use type_access::service::register::create_type_access;

        check_authorized(cxt)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_type_access(&args, conn)
    }
}
