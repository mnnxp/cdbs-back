use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::model::{
    Company, CompanyAndRelatedData, ShowCompanyShort, SlimCompany,
};
use crate::models::search::model::ExtraOptions;
use crate::models::search::order::Paginate;
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl SlimCompany {
    /// Get slim company data by company uuid
    pub(crate) fn get_by_uuid(
        company_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<SlimCompany> {
        company_ref::company_ref
            .filter(
                company_ref::uuid
                    .eq(company_uuid)
                    .and(company_ref::is_enabled.eq(true))
                    .and(company_ref::is_delete.eq(false)),
            )
            .select((
                company_ref::uuid,
                company_ref::shortname,
                company_ref::is_supplier,
            ))
            .first::<SlimCompany>(conn)
            .map_err(|err| {
                debug!("Failed get slim company: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl Company {
    /// Get company data from company_ref table by uuid
    pub(crate) fn get_company_by_uuid(
        target_company_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<Company> {
        company_ref::company_ref
            .filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::is_enabled.eq(true))
                    .and(company_ref::is_delete.eq(false)),
            )
            .select((
                company_ref::uuid,
                company_ref::orgname,
                company_ref::shortname,
                company_ref::inn,
                company_ref::phone,
                company_ref::email,
                company_ref::description,
                company_ref::address,
                company_ref::site_url,
                company_ref::time_zone,
                company_ref::user_uuid,
                company_ref::image_file_uuid,
                company_ref::region_id,
                company_ref::company_type_id,
                company_ref::type_access_id,
                company_ref::is_supplier,
                company_ref::is_email_verified,
                company_ref::created_at,
                company_ref::updated_at,
            ))
            .first::<Company>(conn)
            .map_err(|err| {
                debug!("Failed get company: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl ShowCompanyShort {
    /// Gets company short data by company_uuid wtihout check access
    pub(crate) fn get_without_check_by_uuid(
        target_company_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowCompanyShort> {
        Company::get_company_by_uuid(target_company_uuid, conn).map(|c| c.into())
    }

    /// Gets companies short data by vec uuids
    pub(crate) fn get_list_by_uuids(
        companies_uuids: &[Uuid],
        supplier: &bool,
        options: &ExtraOptions,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        if companies_uuids.is_empty() {
            return Ok(Vec::new());
        }

        // First, filter UUIDs by access permission
        let mut accessible_uuids = Vec::new();
        for &uuid in companies_uuids {
            if require_permission(
                &options.logged_user_uuid,
                AccessEntity::Company,
                &uuid,
                AccessOperation::Read,
                conn,
            )
            .is_ok()
            {
                accessible_uuids.push(uuid);
            }
        }

        if accessible_uuids.is_empty() {
            return Ok(Vec::new());
        }

        // Then query only accessible companies
        let mut query = company_ref::company_ref
            .filter(company_ref::uuid.eq_any(&accessible_uuids))
            .filter(company_ref::is_enabled.eq(true))
            .filter(company_ref::is_delete.eq(false))
            .into_boxed();

        if *supplier {
            query = query.filter(company_ref::is_supplier.eq(true));
        }

        query
            .select((
                company_ref::uuid,
                company_ref::shortname,
                company_ref::inn,
                company_ref::description,
                company_ref::image_file_uuid,
                company_ref::region_id,
                company_ref::company_type_id,
                company_ref::type_access_id,
                company_ref::is_supplier,
                company_ref::created_at,
                company_ref::updated_at,
            ))
            .limit(paginate.limit)
            .offset(paginate.offset)
            .load::<ShowCompanyShort>(conn)
            .map_err(|err| {
                debug!("Failed get companies by uuids: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Gets all public companies short data
    pub(crate) fn get_all_public(
        supplier: &bool,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        let mut query = company_ref::company_ref.into_boxed();
        query = match supplier {
            true => query.filter(
                company_ref::type_access_id
                    .eq(3)
                    .and(company_ref::is_supplier.eq(true))
                    .and(company_ref::is_enabled.eq(true))
                    .and(company_ref::is_delete.eq(false)),
            ),
            false => query.filter(
                company_ref::type_access_id
                    .eq(3)
                    .and(company_ref::is_enabled.eq(true))
                    .and(company_ref::is_delete.eq(false)),
            ),
        };

        let target_companies_uuids = query
            .select(company_ref::uuid)
            .limit(paginate.limit)
            .offset(paginate.offset)
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed get public companies: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut result = Vec::new();
        for target_company_uuid in target_companies_uuids.iter() {
            result.push(ShowCompanyShort::get_without_check_by_uuid(
                target_company_uuid,
                conn,
            )?);
        }
        Ok(result)
    }
}

impl CompanyAndRelatedData {
    /// Collecting company data and related data using uuid
    pub(crate) fn get_by_uuid(
        target_company_uuid: &Uuid,
        options: &ExtraOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<CompanyAndRelatedData> {
        require_permission(
            &options.logged_user_uuid,
            AccessEntity::Company,
            target_company_uuid,
            AccessOperation::Read,
            conn,
        )?;
        Company::get_company_by_uuid(target_company_uuid, conn).map(|c| c.into())
    }

    /// Collecting supplier data and related data using UUID.
    /// Only publicly available data is selected without access verification.
    pub(crate) fn get_supplier_by_uuid(
        target_company_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<CompanyAndRelatedData> {
        // collect data for company
        let company: Company = Company::get_company_by_uuid(target_company_uuid, conn)?;

        // checking access type and supplier status of the company
        if company.type_access_id != 3 || !company.is_supplier {
            debug!(
                "No suitable supplier: {:?}, {:?}",
                company.type_access_id, company.is_supplier
            );
            return Err(get_err_msg(ErrorMessage::NoSuitableSupplierHasBeenFound));
        }

        Ok(company.into())
    }
}
