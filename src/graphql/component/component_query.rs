use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::errors::ServiceResult;
use crate::models::component::component_modification::file_to_set_modification as component_modification_file_to_set_modification;
use crate::models::component::component_modification::file_to_set_modification::model::FileToSetModification;
use crate::models::component::model::{ComponentAndRelatedData, ShowComponentShort};
use crate::models::component::service as component;
use crate::models::user;

#[derive(Default)]
pub struct ComponentQuery;

#[Object]
impl ComponentQuery {
    async fn components(
        &self,
        cxt: &Context<'_>,
        components_uuids: Vec<String>,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        // authorization check
        let logged_uuid_user: Uuid = user::get_logged_uuid_user(cxt, true)?;

        let mut target_uuids_components: Vec<Uuid> = Vec::new();
        for x in components_uuids.iter() {
            target_uuids_components.push(Uuid::parse_str(x).unwrap());
        }

        component::list::find_components(cxt, &target_uuids_components, &logged_uuid_user)
    }

    async fn component(
        &self,
        cxt: &Context<'_>,
        uuid_component: String,
    ) -> ServiceResult<ComponentAndRelatedData> {
        // authorization check
        let logged_uuid_user: Uuid = user::get_logged_uuid_user(cxt, true)?;

        component::list::find_uuid_component(
            cxt,
            &Uuid::parse_str(&uuid_component)?,
            &logged_uuid_user,
        )
    }

    async fn files_set_modification(
        &self,
        cxt: &Context<'_>,
        id_set: Option<i32>,
        // id_set: Option<i32>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<FileToSetModification>> {
        use component_modification_file_to_set_modification::service::list::get_files_set_modification;
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_set: i32 = id_set.unwrap_or(0);
        // let id_set: i32 = id_set.unwrap_or_(0);
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        get_files_set_modification(cxt, id_set, limit, offset)
    }
}
