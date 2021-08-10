use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use async_graphql::Context;
use crate::models::component::model::Component;
use crate::models::component::model::ShowComponent;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn find_components(
    context: &Context<'_>,
    target_uuid_component: Vec<Uuid>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Component>> {
    use crate::schema::component_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    let mut query = component_ref.into_boxed();

    if !target_uuid_component.is_empty() {
        query = query.filter(uuid.eq_any(target_uuid_component))
    }

    Ok(query
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Component>(conn)?)
}

pub(crate) fn find_uuid_component(
    context: &Context<'_>,
    target_uuid_component: Uuid,
) -> ServiceResult<ShowComponent> {
    use crate::models::component::param::model::ParamComponent;
    use crate::models::component::license::model::LicenseComponent;
    use crate::models::component::license::model::License;
    use crate::models::component::file::model::FileComponent;
    use crate::models::file::model::ShowFile;
    use crate::models::component::component_modification::model::ComponentModification;
    use crate::models::component::component_modification::param::model::ParamModification;
    use crate::schema::component_ref::dsl as component_ref;
    use crate::schema::license_ref::dsl as license_ref;
    use crate::schema::file_ref::dsl as file_ref;
    let conn: &PooledConnection = &get_conn(context)?;

    let component = component_ref::component_ref
        .filter(component_ref::uuid.eq(target_uuid_component))
        .first::<Component>(conn)
        .expect("Error loading component");
    let param_component = ParamComponent::belonging_to(&component)
        .load::<ParamComponent>(conn)
        .expect("Error loading param_component");
    let license = LicenseComponent::belonging_to(&component)
        .load::<LicenseComponent>(conn)
        .expect("Error loading license to component");
    let license_id: Vec<i32> = license.into_iter()
        .map(|x| x.id_license)
        .collect();
    let license = license_ref::license_ref
        .filter(license_ref::id.eq_any(license_id))
        .load::<License>(conn)
        .expect("Error loading license");
    let file = FileComponent::belonging_to(&component)
        .load::<FileComponent>(conn)
        .expect("Error loading file to component");
    let file_uuid: Vec<Uuid> = file.into_iter()
        .map(|x| x.uuid_file)
        .collect();
    let file = file_ref::file_ref
        .filter(file_ref::uuid.eq_any(file_uuid))
        .select((
            file_ref::uuid,
            file_ref::uuid_file_parent,
            file_ref::uuid_user,
            file_ref::filename,
            file_ref::content_type,
            file_ref::id_ext,
            file_ref::filesize,
            file_ref::path_file,
            file_ref::created_at,
            file_ref::updated_at,
        ))
        .load::<ShowFile>(conn)
        .expect("Error loading files");
    let component_modification = ComponentModification::belonging_to(&component)
        .load::<ComponentModification>(conn)
        .expect("Error loading component_modification");
    let param_modification = ParamModification::belonging_to(&component_modification)
        .load::<ParamModification>(conn)
        .expect("Error loading param_modification");
    let result = ShowComponent {
        component,
        param_component,
        license,
        file,
        component_modification,
        param_modification
    };
    Ok(result)
}
