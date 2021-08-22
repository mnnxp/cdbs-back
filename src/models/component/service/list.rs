use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use async_graphql::Context;
use crate::models::component::model::{Component, ComponentAndRelatedData};
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
) -> ServiceResult<ComponentAndRelatedData> {
    use crate::models::component::actual_status::model::ActualStatusTranslateList;
    use crate::models::component::component_type::model::ComponentTypeTranslateList;
    use crate::models::component::param::model::{ParamComponent, ComponentParamWithTranslation};
    use crate::models::component::license::model::LicenseComponent;
    use crate::models::component::file::model::FileComponent;
    use crate::models::component::keyword::model::KeywordComponent;
    use crate::models::component::spec::model::{SpecComponent, ComponentSpecWithTranslation};
    use crate::models::component::supplier::model::{SupplierComponent, ComponentSupplierRelatedData};
    use crate::models::company::model::SlimCompany;
    use crate::models::component::component_modification::model::{
        ComponentModification,
        ComponentModificationWithActualStatus,
        ComponentModificationAndRelatedData,
    };
    use crate::models::component::component_modification::param::model::{
        ParamModification, ParamModificationRelate
    };
    use crate::models::component::component_modification::set_of_files_program::model::{
        SetOfFilesProgram, SetOfFilesProgramRelatedData
    };
    use crate::models::relate_ref::program::model::Program;
    use crate::models::relate_ref::param::model::ParamTranslateList;
    use crate::models::relate_ref::license::model::License;
    use crate::models::relate_ref::keyword::model::Keyword;
    use crate::models::relate_ref::file::model::ShowFile;
    use crate::models::relate_ref::spec::model::SpecTranslateList;
    use crate::schema::component_ref::dsl as component_ref;
    use crate::schema::param_translate_list::dsl as param_translate_list;
    use crate::schema::component_type_translate_list::dsl as component_type_translate_list;
    use crate::schema::actual_status_translate_list::dsl as actual_status_translate_list;
    use crate::schema::spec_translate_list::dsl as spec_translate_list;
    use crate::schema::license_ref::dsl as license_ref;
    use crate::schema::keyword_ref::dsl as keyword_ref;
    use crate::schema::company_ref::dsl as company_ref;
    use crate::schema::program_ref::dsl as program_ref;
    use crate::schema::file_ref::dsl as file_ref;
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    // collect data for component
    let component: Component = component_ref::component_ref
        .filter(component_ref::uuid.eq(target_uuid_component))
        .first::<Component>(conn)
        .expect("Error loading component");
    let slim_user = crate::models::user::get_slim_user_from_uuid(&component.uuid_user, conn)
        .expect("Error loading slim_user data");
    let component_type: ComponentTypeTranslateList = component_type_translate_list::component_type_translate_list
        .filter(component_type_translate_list::id_component_type.eq(component.id_component_type)
        .and(component_type_translate_list::id_lang.eq(set_id_lang)))
        .first::<ComponentTypeTranslateList>(conn)
        .expect("Error loading component_type_ref");
    let actual_status: ActualStatusTranslateList = actual_status_translate_list::actual_status_translate_list
        .filter(actual_status_translate_list::id_actual_status.eq(component.id_actual_status)
        .and(actual_status_translate_list::id_lang.eq(set_id_lang)))
        .first::<ActualStatusTranslateList>(conn)
        .expect("Error loading actual_status_ref");

    let param_component: Vec<ParamComponent> = ParamComponent::belonging_to(&component)
        .load::<ParamComponent>(conn)
        .expect("Error loading param_component");
    let id_param_list: Vec<i32> = param_component
        .iter()
        .map(|x| x.id_param)
        .collect::<Vec<i32>>();
    let param_translate_list: Vec<ParamTranslateList> = param_translate_list::param_translate_list
        .filter(param_translate_list::id_param.eq_any(id_param_list)
        .and(param_translate_list::id_lang.eq(set_id_lang)))
        .load::<ParamTranslateList>(conn)
        .expect("Error loading param_translate_list");

    let mut param_component_with_translate: Vec<ComponentParamWithTranslation> = Vec::new();
    for x in param_component.iter() {
        for y in param_translate_list.iter() {
            if x.id_param == y.id_param {
                let res: ComponentParamWithTranslation = (x.to_owned(),y.clone()).into();
                param_component_with_translate.push(res)
            }
        }
    }

    let license_component = LicenseComponent::belonging_to(&component)
        .load::<LicenseComponent>(conn)
        .expect("Error loading license to component");
    let license_id: Vec<i32> = license_component
        .iter()
        .map(|x| x.id_license)
        .collect::<Vec<i32>>();
    let license = license_ref::license_ref
        .filter(license_ref::id.eq_any(license_id))
        .load::<License>(conn)
        .expect("Error loading license");

    let file_component = FileComponent::belonging_to(&component)
        .load::<FileComponent>(conn)
        .expect("Error loading file to component");
    let file_uuid: Vec<Uuid> = file_component
        .iter()
        .map(|x| x.uuid_file)
        .collect::<Vec<Uuid>>();
    let component_file = file_ref::file_ref
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

    let spec_component: Vec<SpecComponent> = SpecComponent::belonging_to(&component)
        .load::<SpecComponent>(conn)
        .expect("Error loading spec_component");
    let id_spec_list: Vec<i32> = spec_component
        .iter()
        .map(|x| x.id_spec)
        .collect::<Vec<i32>>();
    let spec_translate_list: Vec<SpecTranslateList> = spec_translate_list::spec_translate_list
        .filter(spec_translate_list::id_spec.eq_any(id_spec_list)
        .and(spec_translate_list::id_lang.eq(set_id_lang)))
        .load::<SpecTranslateList>(conn)
        .expect("Error loading spec_translate_list");

    let mut spec_component_with_translate: Vec<ComponentSpecWithTranslation> = Vec::new();
    for x in spec_component.iter() {
        for y in spec_translate_list.iter() {
            if x.id_spec == y.id_spec {
                let res: ComponentSpecWithTranslation = (x.to_owned(),y.clone()).into();
                spec_component_with_translate.push(res)
            }
        }
    }

    let keyword_component: Vec<KeywordComponent> = KeywordComponent::belonging_to(&component)
        .load::<KeywordComponent>(conn)
        .expect("Error loading keyword_component");

    let id_keyword_list: Vec<i32> = keyword_component
        .iter()
        .map(|x| x.id_keyword)
        .collect::<Vec<i32>>();

    let keyword_component: Vec<Keyword> = keyword_ref::keyword_ref
        .filter(keyword_ref::id.eq_any(id_keyword_list))
        .load::<Keyword>(conn)
        .expect("Error loading spec_translate_list");

    // collect data for modifications the component
    let component_modification: Vec<ComponentModification> = ComponentModification::belonging_to(&component)
        .load::<ComponentModification>(conn)
        .expect("Error loading component_modification");

    // debug!("Component modification component_modification: {:#?}", component_modification);

    let id_component_modification: Vec<i32> = component_modification
        .iter()
        .map(|x| x.id_actual_status)
        .collect::<Vec<i32>>();

    let actual_status_modification: Vec<ActualStatusTranslateList> = actual_status_translate_list::actual_status_translate_list
        .filter(actual_status_translate_list::id_actual_status.eq_any(id_component_modification)
        .and(actual_status_translate_list::id_lang.eq(set_id_lang)))
        .load::<ActualStatusTranslateList>(conn)
        .expect("Error loading actual_status_ref");

    // debug!("Component modification actual_status_modification: {:#?}", actual_status_modification);

    let mut component_modification_with_status: Vec<ComponentModificationWithActualStatus> = Vec::new();
    for x in component_modification.iter() {
        for y in actual_status_modification.iter() {
            if x.id_actual_status == y.id_actual_status {
                let res: ComponentModificationWithActualStatus = (x.clone(),y.clone()).into();
                component_modification_with_status.push(res)
            }
        }
    }

    // debug!("Component modification component_modification_with_status: {:#?}", component_modification_with_status);

    let param_component_modification: Vec<Vec<ParamModification>> = ParamModification::belonging_to(&component_modification)
        .load::<ParamModification>(conn)
        .expect("Error loading param_component_modification")
        .grouped_by(&component_modification);

    // debug!("Component modification param_component_modification: {:#?}", param_component_modification);

    let mut id_param_component_modification: Vec<i32> = Vec::new();
    for x in param_component_modification.iter() {
        for y in x.iter() {
            id_param_component_modification.push(y.id_param);
        }
    }

    let param_translate_list: Vec<ParamTranslateList> = param_translate_list::param_translate_list
        .filter(param_translate_list::id_param.eq_any(id_param_component_modification)
        .and(param_translate_list::id_lang.eq(set_id_lang)))
        .load::<ParamTranslateList>(conn)
        .expect("Error loading param_translate_list");

    let mut param_component_modification_with_translate: Vec<Vec<ParamModificationRelate>> = Vec::new();
    for w in param_component_modification.iter() {
        for x in w.iter() {
            let mut vec_values: Vec<ParamModificationRelate> = Vec::new();
            for y in param_translate_list.iter() {
                if x.id_param == y.id_param {
                    let res: ParamModificationRelate = (x.to_owned(),y.clone()).into();
                    vec_values.push(res)
                }
            }
            param_component_modification_with_translate.push(vec_values)
        }
    }

    // debug!("Component modification param_component_modification_with_translate: {:#?}", param_component_modification_with_translate);

    let set_files_program_for_modification: Vec<Vec<SetOfFilesProgram>> = SetOfFilesProgram::belonging_to(&component_modification)
        .load::<SetOfFilesProgram>(conn)
        .expect("Error loading set_files_program_for_modification")
        .grouped_by(&component_modification);

    // debug!("Component modification set_files_program_for_modification: {:#?}", set_files_program_for_modification);

    let mut id_program_for_set: Vec<i32> = Vec::new();
    for x in set_files_program_for_modification.iter() {
        for y in x.iter() {
            id_program_for_set.push(y.id_program);
        }
    }

    let program_for_set_files: Vec<Program> = program_ref::program_ref
        .filter(program_ref::id.eq_any(id_program_for_set))
        .load::<Program>(conn)
        .expect("Error loading program_ref");

    let mut set_files_program_with_relate: Vec<Vec<SetOfFilesProgramRelatedData>> = Vec::new();
    for w in set_files_program_for_modification.iter() {
        for x in w.iter() {
            let mut vec_values: Vec<SetOfFilesProgramRelatedData> = Vec::new();
            for y in program_for_set_files.iter() {
                if x.id_program == y.id {
                    let res: SetOfFilesProgramRelatedData = (x.to_owned(),y.clone()).into();
                    vec_values.push(res)
                }
            }
            set_files_program_with_relate.push(vec_values)
        }
    }

    // debug!("Component modification set_files_program_component_modification: {:#?}", set_files_program_component_modification);

    let mut component_modification_with_relate: Vec<ComponentModificationAndRelatedData> = Vec::new();

    for w in component_modification_with_status.iter() {
        let mut vec_values_set: Vec<SetOfFilesProgramRelatedData> = Vec::new();
        for x in set_files_program_with_relate.iter() {
            for y in x.iter() {
                if w.modification.uuid == y.uuid_modification {
                    vec_values_set.push(y.to_owned())
                }
            }
        }
        let mut vec_values_param: Vec<ParamModificationRelate> = Vec::new();
        for x in param_component_modification_with_translate.iter() {
            for y in x.iter() {
                if w.modification.uuid == y.uuid_modification {
                    vec_values_param.push(y.to_owned())
                }
            }
        }
        component_modification_with_relate.push((
            w.clone(),
            vec_values_set,
            vec_values_param
        ).into())
    }

    // debug!("Component component_modification_with_relate: {:#?}", component_modification_with_relate);

    // collect data for supplier component
    let supplier_component: Vec<SupplierComponent> = SupplierComponent::belonging_to(&component)
        .load::<SupplierComponent>(conn)
        .expect("Error loading supplier_component");

    let uuid_supplier_list: Vec<Uuid> = supplier_component
        .iter()
        .map(|x| x.uuid_company)
        .collect::<Vec<Uuid>>();

    let slim_company_supplier: Vec<SlimCompany> = company_ref::company_ref
        .filter(company_ref::uuid.eq_any(uuid_supplier_list))
        .select((
            company_ref::uuid,
            company_ref::shortname,
            company_ref::is_supplier,
        ))
        .load::<SlimCompany>(conn)
        .expect("Error loading supplier_component");

    let mut supplier_component_with_relate: Vec<ComponentSupplierRelatedData> = Vec::new();
    for x in supplier_component.iter() {
        for y in slim_company_supplier.iter() {
            if x.uuid_company == y.uuid {
                let res: ComponentSupplierRelatedData = (x.clone(),y.clone()).into();
                supplier_component_with_relate.push(res)
            }
        }
    }

    let result = ComponentAndRelatedData {
        uuid: (component.uuid),
        uuid_component_parent: (component.uuid_component_parent),
        name: (component.name),
        description: (component.description),
        slim_user: (slim_user),
        id_type_access: (component.id_type_access),
        component_type: (component_type),
        actual_status: (actual_status),
        is_standard: (component.is_standard),
        updated_at: (component.updated_at),
        param_component: (param_component_with_translate),
        license: (license),
        file: (component_file),
        spec_component: (spec_component_with_translate),
        keyword_component: (keyword_component),
        component_modification: (component_modification_with_relate),
        supplier_component: (supplier_component_with_relate),
    };

    debug!("Component data: {:#?}", result);

    Ok(result)
}
