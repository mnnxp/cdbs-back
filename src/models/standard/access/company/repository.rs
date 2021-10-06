use crate::errors::ServiceResult;
use crate::models::standard::access::company::model::{
    CompanyAccessStandard, CompanyAccessStandardAndRelatedData
};
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::company_access_to_standard::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

impl CompanyAccessStandardAndRelatedData {
    /// Collect related data for companies lits access standard
    pub(crate) fn from_standard_by_uuid(
        target_standard_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyAccessStandardAndRelatedData>> {
        let list_companies_with_access = company_access_to_standard
            .filter(standard_uuid.eq(target_standard_uuid))
            .load::<CompanyAccessStandard>(conn)?;

        let mut target_types_access_ids: Vec<i32> = Vec::new();
        for x in list_companies_with_access.iter() {
            target_types_access_ids.push(x.type_access_id.to_owned());
        }

        let type_access_with_translate = TypeAccessTranslateList::get_types_access_by_ids(
            &target_types_access_ids,
            set_lang_id,
            conn
        )?;

        let mut res: Vec<CompanyAccessStandardAndRelatedData> = Vec::new();
        for x in list_companies_with_access {
            for type_access in &type_access_with_translate {
                if x.type_access_id == type_access.type_access_id {
                    res.push(
                        CompanyAccessStandardAndRelatedData{
                            standard_uuid: x.standard_uuid,
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
