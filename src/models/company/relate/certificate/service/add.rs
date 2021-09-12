use crate::errors::{
    // ServiceError,
    ServiceResult
};
use crate::database::PooledConnection;
use crate::models::company::certificate::model::{
    CompanyCertificate,
    IptCompanyCertificateData,
    InsertableCompanyCertificate,
};
// use crate::models::user::model::TargetUser;
use crate::models::relate_ref::file::model::{
    ListObject, IptPreliminaryFileData, PreliminaryFileData
};
use crate::models::relate_ref::file as file;
use crate::schema::company_certificate_ref::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_certificate(
    target_user: Uuid,
    cert_data: IptCompanyCertificateData,
    file_data: IptPreliminaryFileData,
    conn: &PooledConnection,
) -> ServiceResult<String> {
    // let pool = pool.clone();
    // let conn = pool.get().unwrap();

    // let content_sha1: String = file_data.sha1.clone();

    let preliminary_file_data = PreliminaryFileData::from_ipt_preliminary_file_data(
        target_user,
        Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?, // <-- todo!(get uuid default file)
        ListObject::CompanyCertificate(cert_data.company_uuid),
        file_data,
        conn
    )?;

    let slim_file = file::service::register::register(
        preliminary_file_data,
        conn
    )?;

    // workaround until i figure make the pre-url generation
    let temp_string = format!("This will be url for upload file {:?}", slim_file.path_file);

    let new_company_certificate = InsertableCompanyCertificate{
        file_uuid: slim_file.uuid,
        company_uuid: cert_data.company_uuid,
        description: cert_data.description,
    };

    // debug!("fn create_favorite START SEARCH ={:?}", flag_found_favorite);

    let company_inserted_certificate: CompanyCertificate = diesel::insert_into(company_certificate_ref)
        .values(new_company_certificate)
        .get_result(conn)?;

    debug!("Company inserted certificate: {:?}", company_inserted_certificate);

    // todo!(presigned_url)
    Ok(temp_string)
}
