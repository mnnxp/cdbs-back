use async_graphql::{Context, Enum, Object, SimpleObject};
use uuid::Uuid;

use crate::auth::access::{
    check_company_access, check_component_access, check_service_access, check_standard_access,
    AccessInfo as AuthAccessInfo, AccessSource as AuthAccessSource,
};
use crate::auth::AuthContext;
use crate::database::get_conn;
use crate::errors::ServiceResult;

#[derive(SimpleObject, Debug)]
pub struct AccessCheckResult {
    /// Whether the user has any level of access to the object
    pub has_access: bool,
    /// Numeric access level: 1=full, 2=write, 3=read, null=none
    pub access_level: Option<i32>,
    /// Source of the access: OWNER, DIRECT_ACCESS, COMPANY_ROLE, PUBLIC, NONE
    pub source: AccessSource,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum AccessSource {
    Owner,
    DirectAccess,
    CompanyRole,
    Public,
    None,
}

impl From<AuthAccessSource> for AccessSource {
    fn from(s: AuthAccessSource) -> Self {
        match s {
            AuthAccessSource::Owner => AccessSource::Owner,
            AuthAccessSource::DirectAccess => AccessSource::DirectAccess,
            AuthAccessSource::CompanyRole => AccessSource::CompanyRole,
            AuthAccessSource::Public => AccessSource::Public,
            AuthAccessSource::None => AccessSource::None,
        }
    }
}

impl From<AuthAccessInfo> for AccessCheckResult {
    fn from(info: AuthAccessInfo) -> Self {
        AccessCheckResult {
            has_access: info.has_access,
            access_level: info.access_level,
            source: info.source.into(),
        }
    }
}

#[derive(Default)]
pub struct UserAccessQuery;

#[Object]
impl UserAccessQuery {
    /// Get current user's access information for a specific component
    async fn my_access_to_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<AccessCheckResult> {
        let user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();
        let mut conn = get_conn(cxt)?;
        let info = check_component_access(&user_uuid, &component_uuid, &mut conn)?;
        Ok(info.into())
    }

    /// Get current user's access information for a specific standard
    async fn my_access_to_standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<AccessCheckResult> {
        let user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();
        let mut conn = get_conn(cxt)?;
        let info = check_standard_access(&user_uuid, &standard_uuid, &mut conn)?;
        Ok(info.into())
    }

    /// Get current user's access information for a specific service
    async fn my_access_to_service(
        &self,
        cxt: &Context<'_>,
        service_uuid: Uuid,
    ) -> ServiceResult<AccessCheckResult> {
        let user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();
        let mut conn = get_conn(cxt)?;
        let info = check_service_access(&user_uuid, &service_uuid, &mut conn)?;
        Ok(info.into())
    }

    /// Get current user's access information for a specific company
    async fn my_access_to_company(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<AccessCheckResult> {
        let user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();
        let mut conn = get_conn(cxt)?;
        let info = check_company_access(&user_uuid, &company_uuid, &mut conn)?;
        Ok(info.into())
    }
}
