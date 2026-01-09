use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::check_company_access;
use crate::models::company::company_represent::model::{
    CompanyRepresentAndRelatedData, CompanyRepresentsArg,
};
use crate::models::search::order::Paginate;
use crate::schema::company_represent_ref::dsl as company_represent_ref;
use diesel::{prelude::*, PgConnection};
use uuid::Uuid;

/// Возвращает информацию о представительствах компании.
pub(crate) fn get_represents(
    logged_user_uuid: &Uuid,
    args: &CompanyRepresentsArg,
    // sort: &Sort,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
    let mut args: CompanyRepresentsArg = args.clone();
    if args.company_uuid.is_nil() {
        match args.represents_uuids.first() {
            Some(represent_uuid) => {
                let company_uuid = get_company_of_represent(represent_uuid, conn)?;
                args.set_company_uuid(&company_uuid);
            }
            None => {
                return Err(get_err_msg(
                    ErrorMessage::NeedToChooseCompanyOrRepresentative,
                ))
            }
        }
    }

    let need_access_level = 3;
    check_company_access(
        logged_user_uuid,
        &args.company_uuid,
        need_access_level,
        conn,
    )?;

    if args.represents_uuids.is_empty() {
        let represents_uuids = get_represents_company_uuid(&args.company_uuid, conn)?;
        if represents_uuids.is_empty() {
            // return empty array if not found represents
            return Ok(Vec::new());
        }
        args.set_represents_uuids(represents_uuids);
    }
    CompanyRepresentAndRelatedData::get_by_args(&args, paginate, conn)
}

/// Get represents Uuids for company by uuid
pub(crate) fn get_represents_company_uuid(
    company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    company_represent_ref::company_represent_ref
        .filter(company_represent_ref::company_uuid.eq(company_uuid))
        .select(company_represent_ref::uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get company uuid by represent uuid: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Get company Uuid by represent uuid
pub(crate) fn get_company_of_represent(
    represent_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    company_represent_ref::company_represent_ref
        .filter(company_represent_ref::uuid.eq(represent_uuid))
        .select(company_represent_ref::company_uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get company uuid by represent uuid: {:?}", err);
            ServiceError::InternalServerError
        })
}
