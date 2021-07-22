// use crate::user::model::{LoggedUser, User};
use crate::schema::*;
// use chrono::*;
use uuid::Uuid;
// use num::ToPrimitive;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Param {
    pub id: i32,
    pub paramname: String,
}

#[derive(Debug, Insertable)]
#[table_name = "param_ref"]
pub struct InsertableParam {
    pub paramname: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ParamData {
    pub paramname: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ParamToModel {
    pub id: i32,
    pub uuid: Uuid,
    pub id_param: i32,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ParamToModelData {
    pub uuid: Uuid,
    pub id_param: i32,
    pub value: String,
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
