use crate::errors::{ServiceResult, ServiceError};
use crate::database::{get_conn, PooledConnection};
use crate::models::user::model::ShowUser;
use async_graphql::{
    dataloader::DataLoader, dataloader::Loader, Context, EmptySubscription, FieldResult, Schema,
};
use diesel::*;
use uuid::Uuid;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct FileLoader;

// #[async_trait]
// impl Loader<Favicon> for FileLoader {
//     type Value = String;
//     type Error = ServiceError;
//     async fn load(
//         &self,
//         keys: &[Uuid],
//     ) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
//         // Turn keys into strings for database request.
//         let keys: Vec<_> = keys.iter().map(|x| &x.0 as &str).collect();
//         let items: Vec<Self::Value> =
//             find_all_keys(self.client.collection(Self::DOCUMENTS), keys).await?;
//         Ok(items
//             .into_iter()
//             .map(|tag| (tag.meta.id.clone(), tag))
//             .collect())
//     }
// }
