use crate::schema::*;
use async_graphql::*;

/// RoleAccess models
#[derive(Debug, Insertable)]
#[diesel(table_name = role_access)]
pub(crate) struct InsertableRoleAccess {
    pub(crate) role_id: i32,
    pub(crate) type_access_id: i32,
}

/// Data for requesting to add access to an existing role
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptRoleAccessData {
    /// Role ID
    pub(crate) role_id: i32,
    /// Access type identifiers (list)
    pub(crate) types_access_ids: Vec<i32>,
}

impl From<&IptRoleAccessData> for Vec<InsertableRoleAccess> {
    fn from(ipt_data: &IptRoleAccessData) -> Vec<InsertableRoleAccess> {
        let IptRoleAccessData {
            role_id,
            types_access_ids,
            ..
        } = ipt_data;

        let mut res: Vec<InsertableRoleAccess> = Vec::new();

        for tai in types_access_ids {
            res.push(InsertableRoleAccess {
                role_id: *role_id,
                type_access_id: *tai,
            })
        }

        res // <-- returning an array of parsed records
    }
}

/// Data for deleting access levels for a role
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct DelRoleAccessData {
    /// Role identifier
    pub(crate) role_id: i32,
    /// Identifiers of access types to be deleted (list)
    pub(crate) types_access_ids: Vec<i32>,
}

// #[derive(Debug, Deserialize, Clone, Default, SimpleObject)]
// pub(crate) struct RoleAccessAndRelatedData {
//     pub(crate) role: RoleAccessTranslateList,
//     pub(crate) access: TypeAccessTranslateList,
// }
