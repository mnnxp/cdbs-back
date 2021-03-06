// use crate::user::model::{LoggedUser, User};
use crate::schema::*;
use chrono::*;
// use uuid::Uuid;
// use num::ToPrimitive;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct File {
    pub id: i32,
    pub id_file: i32,
    // pub hash: Vec<u8>,
    pub id_user_create: i32,
    pub created_at: NaiveDateTime,
    pub filename: String,
    pub id_ext: i32,
    pub filesize: f64,
    pub path_file: String,
}

#[derive(Debug, Insertable)]
#[table_name = "file_ref"]
pub struct InsertableFile {
    pub id_file: i32,
    // pub hash: Vec<u8>,
    pub id_user_create: i32,
    pub created_at: NaiveDateTime,
    pub filename: String,
    pub id_ext: i32,
    pub filesize: f64,
    pub path_file: String,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct DataFile {
    pub id_file: i32,
    // pub hash: Vec<u8>,
    pub id_user_create: i32,
    pub created_at: NaiveDateTime,
    pub filename: String,
    pub id_ext: i32,
    pub filesize: f64,
    pub path_file: String,
}

impl From<DataFile> for InsertableFile {
    fn from(name_for_file: DataFile) -> Self {
        let DataFile {
            id_file,
            filename,
            path_file,
            ..
        } = name_for_file;

        let id_ext = 1; // get_ext_id(&path_file);
        // let hash = Vec::from("76738cf561df624bff0de7151eec68c1d40a56c76a8f6859e09c799a251468ac");
        let filesize = 156.5; // get_file_size(&path_file);
        let id_user_create= 1;

        Self {
            id_file,
            // hash,
            id_user_create,
            created_at: chrono::Local::now().naive_local(),
            filename,
            id_ext,
            filesize,
            path_file,
        }
    }
}
