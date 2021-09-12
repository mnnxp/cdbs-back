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
        let logged_user_uuid: Uuid = user::get_logged_user_uuid(cxt, true)?;

        let mut target_uuids_components: Vec<Uuid> = Vec::new();
        for x in components_uuids.iter() {
            target_uuids_components.push(Uuid::parse_str(x).unwrap());
        }

        component::list::find_components(cxt, &target_uuids_components, &logged_user_uuid)
    }

    async fn component(
        &self,
        cxt: &Context<'_>,
        component_uuid: String,
    ) -> ServiceResult<ComponentAndRelatedData> {
        // authorization check
        let logged_user_uuid: Uuid = user::get_logged_user_uuid(cxt, true)?;

        component::list::find_component_uuid(
            cxt,
            &Uuid::parse_str(&component_uuid)?,
            &logged_user_uuid,
        )
    }

    async fn files_set_modification(
        &self,
        cxt: &Context<'_>,
        set_id: Option<i32>,
        // set_id: Option<i32>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<FileToSetModification>> {
        use component_modification_file_to_set_modification::service::list::get_files_set_modification;
        // authorization check
        user::util::check_authorized(cxt)?;

        let set_id: i32 = set_id.unwrap_or(0);
        // let set_id: i32 = set_id.unwrap_or_(0);
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        get_files_set_modification(cxt, set_id, limit, offset)
    }
}
