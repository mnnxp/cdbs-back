use crate::schema::*;
use crate::models::company::member::role::model::RoleMember;
use crate::models::relate_ref::type_access::model::TypeAccess;
use async_graphql::*;

// RoleAccess models
#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[diesel(primary_key(role_id, type_access_id))]
#[diesel(belongs_to(RoleMember, foreign_key = role_id))]
#[diesel(belongs_to(TypeAccess, foreign_key = type_access_id))]
#[diesel(table_name = role_access)]
pub(crate) struct RoleAccess {
    pub(crate) role_id: i32,
    pub(crate) type_access_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = role_access)]
pub(crate) struct InsertableRoleAccess {
    pub(crate) role_id: i32,
    pub(crate) type_access_id: i32,
}

#[derive(Debug, Serialize, SimpleObject)]
pub(crate) struct ShowRoleAccess {
    pub(crate) role_id: i32,
    pub(crate) types_access_ids: Vec<i32>,
}

impl From<&[RoleAccess]> for ShowRoleAccess {
    fn from(roles_data: &[RoleAccess]) -> ShowRoleAccess {
        // set role id for check
        let mut ctrl_role_id = 0;

        // if found role id update set role id
        if let Some(rd) = roles_data.get(0) {
            ctrl_role_id = rd.role_id;
        };

        let mut collect_access_ids: Vec<i32> = Vec::new();

        for ra_d in roles_data {
            // skip if the role id does not match the first role
            if ctrl_role_id == ra_d.role_id {
                collect_access_ids.push(ra_d.type_access_id)
            }
        }

        ShowRoleAccess {
            role_id: ctrl_role_id,
            types_access_ids: collect_access_ids,
        }
    }
}

#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptRoleAccessData {
    pub(crate) role_id: i32,
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

#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct DelRoleAccessData {
    pub(crate) role_id: i32,
    pub(crate) types_access_ids: Vec<i32>,
}

// #[derive(Debug, Deserialize, Clone, Default, SimpleObject)]
// pub(crate) struct RoleAccessAndRelatedData {
//     pub(crate) role: RoleAccessTranslateList,
//     pub(crate) access: TypeAccessTranslateList,
// }
