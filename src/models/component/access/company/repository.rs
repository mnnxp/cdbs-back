use crate::errors::ServiceResult;
use crate::models::component::access::company::model::{
    CompanyAccessComponent, CompanyAccessComponentAndRelatedData
};
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::company_access_to_component::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

impl CompanyAccessComponentAndRelatedData {
    /// Collect related data for companies lits access component
    pub(crate) fn from_component_by_uuid(
        target_component_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyAccessComponentAndRelatedData>> {
        let list_companies_with_access = company_access_to_component
            .filter(component_uuid.eq(target_component_uuid))
            .load::<CompanyAccessComponent>(conn)?;

        let mut target_types_access_ids: Vec<i32> = Vec::new();
        for x in list_companies_with_access.iter() {
            target_types_access_ids.push(x.type_access_id.to_owned());
        }

        let type_access_with_translate = TypeAccessTranslateList::get_types_access_by_ids(
            &target_types_access_ids,
            set_lang_id,
            conn
        )?;

        let mut res: Vec<CompanyAccessComponentAndRelatedData> = Vec::new();
        for x in list_companies_with_access {
            for type_access in &type_access_with_translate {
                if x.type_access_id == type_access.type_access_id {
                    res.push(
                        CompanyAccessComponentAndRelatedData{
                            component_uuid: x.component_uuid,
                            company_uuid: x.company_uuid,
                            type_access: type_access.clone(),
                            is_enabled: x.is_enabled,
                            created_at: x.created_at,
                            updated_at: x.updated_at,
                        }
                    )
                }
            }
        }

        Ok(res)
    }
}
