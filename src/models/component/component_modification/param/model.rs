use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ParamModification {
    pub uuid_modification: Uuid,
    pub id_param: i32,
    pub value: String,
}

#[Object]
impl ParamModification {
    async fn uuid_modification(&self) -> ID {
        self.uuid_modification.into()
    }
    async fn id_param(&self) -> &i32 {
        &self.id_param
    }
    async fn value(&self) -> &String {
        &self.value
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptParamModificationData {
    pub uuid_modification: ID,
    pub id_param: i32,
    pub value: String,
}

#[derive(Debug, Insertable)]
#[table_name = "param_to_modification"]
pub struct InsertableParamModification {
    pub uuid_modification: Uuid,
    pub id_param: i32,
    pub value: String,
}

impl From<IptParamModificationData> for InsertableParamModification {
    fn from(ipt_data: IptParamModificationData) -> Self {
        let IptParamModificationData {
            uuid_modification,
            id_param,
            value,
        } = ipt_data;

        Self {
            uuid_modification: Uuid::parse_str(&uuid_modification.to_string()).unwrap(),
            id_param,
            value,
        }
    }
}
