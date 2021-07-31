use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Param {
    pub id: i32,
    pub paramname: String,
}

#[Object]
impl Param {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn paramname(&self) -> &String {
        &self.paramname
    }
}

#[derive(Debug, Insertable)]
#[table_name = "param_ref"]
pub struct InsertableParam {
    pub paramname: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, InputObject)]
pub struct ParamData {
    pub paramname: String,
}

// #[Object]
// impl ParamData {
//     async fn paramname(&self) -> &String {
//         &self.paramname
//     }
// }

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ParamToModel {
    // pub id: i32,
    pub uuid: Uuid,
    pub id_param: i32,
    pub value: String,
}

#[Object]
impl ParamToModel {
    // async fn id(&self) -> &i32 {
    //     &self.id
    // }
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn id_param(&self) -> &i32 {
        &self.id_param
    }
    async fn value(&self) -> &String {
        &self.value
    }
}


#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptParamToModelData {
    pub uuid: ID,
    pub id_param: i32,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ParamToModelData {
    pub uuid: Uuid,
    pub id_param: i32,
    pub value: String,
}

impl From<IptParamToModelData> for ParamToModelData {
    fn from(ipt_data: IptParamToModelData) -> Self {
        let IptParamToModelData {
            uuid,
            id_param,
            value,
        } = ipt_data;
        ParamToModelData {
            uuid: Uuid::parse_str(&uuid.to_string()).unwrap(),
            id_param,
            value,
        }
    }
}

#[Object]
impl ParamToModelData {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn id_param(&self) -> &i32 {
        &self.id_param
    }
    async fn value(&self) -> &String {
        &self.value
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ShowParamForUuid {
    pub uuid: Uuid,
    pub id_param: i32,
    pub paramname: String,
    pub value: String,
}

#[derive(Debug, Insertable)]
#[table_name = "param_to_component"]
pub struct InsertableParamToComponent {
    pub uuid_component: Uuid,
    pub id_param: i32,
    pub value: String,
}

#[derive(Debug, Insertable)]
#[table_name = "param_to_modification"]
pub struct InsertableParamToModification {
    pub uuid_modification: Uuid,
    pub id_param: i32,
    pub value: String,
}

impl From<Param> for ParamData {
    fn from(file: Param) -> Self {
        let Param {
            paramname,
            ..
        } = file;

        Self {
            paramname,
        }
    }
}

impl From<ParamData> for InsertableParam {
    fn from(data_param: ParamData) -> Self {
        let ParamData {
            paramname,
            ..
        } = data_param;

        Self {
            paramname,
        }
    }
}

impl From<ParamToModelData> for InsertableParamToComponent {
    fn from(data_param_to_model: ParamToModelData) -> Self {
        let ParamToModelData {
            uuid,
            id_param,
            value,
            ..
        } = data_param_to_model;

        let uuid_component = uuid;

        Self {
            uuid_component,
            id_param,
            value,
        }
    }
}

impl From<ParamToModelData> for InsertableParamToModification {
    fn from(data_param_to_model: ParamToModelData) -> Self {
        let ParamToModelData {
            uuid,
            id_param,
            value,
            ..
        } = data_param_to_model;

        let uuid_modification = uuid;

        Self {
            uuid_modification,
            id_param,
            value,
        }
    }
}
