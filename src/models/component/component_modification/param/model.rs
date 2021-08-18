use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::component::param::model::ParamTranslateList;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Debug)]
#[primary_key(uuid_modification)]
#[belongs_to(ComponentModification, foreign_key = "uuid_modification")]
#[belongs_to(ParamTranslateList, foreign_key = "id_param")]
#[table_name = "param_to_modification"]
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

#[derive(Debug, Deserialize, SimpleObject, Description)]
pub struct ParamModificationRelate {
    pub uuid_modification: Uuid,
    pub param: ParamTranslateList,
    pub value: String,
}

impl From<(ParamModification, ParamTranslateList)> for ParamModificationRelate {
    fn from(data: (ParamModification, ParamTranslateList)) -> Self {
        Self {
            uuid_modification: data.0.uuid_modification,
            param: data.1,
            value: data.0.value,
        }
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
