use super::ServiceError;
use super::err_msg::ErrorMessage;

pub(crate) fn value_in_err_msg_en(err_msg: ErrorMessage) -> ServiceError {
    match err_msg {
        ErrorMessage::AccessDenied =>
            ServiceError::BadRequest(String::from("Access denied")),
        ErrorMessage::AccessNotAdded =>
            ServiceError::BadRequest(String::from("Access has not been added")),
        ErrorMessage::AccessNotFoundCompany =>
            ServiceError::BadRequest(String::from("Access not found for company")),
        ErrorMessage::AccessNotFoundUser =>
            ServiceError::BadRequest(String::from("Access not found for user")),
        ErrorMessage::BadFilename =>
            ServiceError::BadRequest(String::from("Bad filename")),
        ErrorMessage::CannotDeleteRows =>
            ServiceError::BadRequest(String::from("Cannot delete rows")),
        ErrorMessage::DataNotFound =>
            ServiceError::BadRequest(String::from("Data not found")),
        ErrorMessage::FoundDuplicateData =>
            ServiceError::BadRequest(String::from("Found duplicate data")),
        ErrorMessage::FoundDuplicateToken =>
            ServiceError::BadRequest(String::from("Found duplicate token")),
        ErrorMessage::DuplicateOfExistingData =>
            ServiceError::BadRequest(String::from("Duplication of existing data was detected")),
        ErrorMessage::ErrorDeleteRole =>
            ServiceError::BadRequest(String::from("Error when deleting a role")),
        ErrorMessage::ErrorIncorrectId =>
            ServiceError::BadRequest(String::from("Error incorrect id")),
        ErrorMessage::ErrorRecordingDependencyData =>
            ServiceError::BadRequest(String::from("Error recording dependency data")),
        ErrorMessage::FailGetUserData =>
            ServiceError::BadRequest(String::from("Fail get SlimUser from Claims")),
        ErrorMessage::FailedAddAccess =>
            ServiceError::BadRequest(String::from("Failed add access for target company")),
        ErrorMessage::FailedCheckData =>
            ServiceError::BadRequest(String::from("Failed check data")),
        ErrorMessage::FailedCheckRole =>
            ServiceError::BadRequest(String::from("Failed check role data")),
        ErrorMessage::FailedCheckSpec =>
            ServiceError::BadRequest(String::from("Failed check spec for standard")),
        ErrorMessage::FailedDeleteAccessForCompany =>
            ServiceError::BadRequest(String::from("Failed delete access for target company")),
        ErrorMessage::FailedDeleteSupplierComponent =>
            ServiceError::BadRequest(String::from("Failed delete company of suppliers component")),
        ErrorMessage::FailedDeleteRelatedStandardsComponent =>
            ServiceError::BadRequest(String::from("Failed delete related standards to component")),
        ErrorMessage::FailedDeleteSuppliersComponent =>
            ServiceError::BadRequest(String::from("Failed delete related suppliers to component")),
        ErrorMessage::FailedGetCompaniesWithAccessComponent =>
            ServiceError::BadRequest(String::from("Failed get companies list have access to component")),
        ErrorMessage::FailedGetCompaniesWithAccessStandard =>
            ServiceError::BadRequest(String::from("Failed get companies list have access to standard")),
        ErrorMessage::FailedMatchArguments =>
            ServiceError::BadRequest(String::from("Failed match arguments")),
        ErrorMessage::FailedSetAccessCompany =>
            ServiceError::BadRequest(String::from("Failed set access for target company")),
        ErrorMessage::FailedUpdateData =>
            ServiceError::BadRequest(String::from("Failed update data")),
        ErrorMessage::FailedUpdateRoleMember =>
            ServiceError::BadRequest(String::from("Failed update role member")),
        ErrorMessage::FailedWriteMetadata =>
            ServiceError::BadRequest(String::from("Failed write metadata")),
        ErrorMessage::FailedRemoveAccessForRole =>
            ServiceError::BadRequest(String::from("Failed: access not delete")),
        ErrorMessage::FileObjectNotFound =>
            ServiceError::BadRequest(String::from("File to object association not found")),
        ErrorMessage::KeywordMustLess =>
            ServiceError::BadRequest(String::from("Keywords must be less than 10 symbols")),
        ErrorMessage::NeedSetUuidOrUsername =>
            ServiceError::BadRequest(String::from("Need set userUuid or username")),
        ErrorMessage::NoActiveFileRevisionFound =>
            ServiceError::BadRequest(String::from("No active file revision found")),
        ErrorMessage::NoSuitableSupplierHasBeenFound =>
            ServiceError::BadRequest(String::from("No suitable supplier has been found")),
        ErrorMessage::NotFoundAccessForRole =>
            ServiceError::BadRequest(String::from("Not found access for target role")),
        ErrorMessage::NotFoundFilename =>
            ServiceError::BadRequest(String::from("Not found filename")),
        ErrorMessage::NotFoundFilesetData =>
            ServiceError::BadRequest(String::from("Not found fileset data")),
        ErrorMessage::NotFoundKeywords =>
            ServiceError::BadRequest(String::from("Not found keywords")),
        ErrorMessage::NotFoundModificationData =>
            ServiceError::BadRequest(String::from("Not found modification data")),
        ErrorMessage::NotFoundParamsForAddingOrUpdaing =>
            ServiceError::BadRequest(String::from("Not found params for adding or updating")),
        ErrorMessage::NotFoundParamsForDeleting =>
            ServiceError::BadRequest(String::from("Not found params for deleting")),
        ErrorMessage::NotFoundRepresentative =>
            ServiceError::BadRequest(String::from("Not found representative")),
        ErrorMessage::NotFoundRolesForCompany =>
            ServiceError::BadRequest(String::from("Not found roles for company")),
        ErrorMessage::NotFoundSpecs =>
            ServiceError::BadRequest(String::from("Not found specs")),
        ErrorMessage::NotFoundStandard =>
            ServiceError::BadRequest(String::from("Not found standard")),
        ErrorMessage::NotFoundTargetFile =>
            ServiceError::BadRequest(String::from("Not found target file")),
        ErrorMessage::NotMorePathInOneQuery =>
            ServiceError::BadRequest(String::from("Not more 100 path in one query")),
        ErrorMessage::PasswordIsNotCorrect =>
            ServiceError::BadRequest(String::from("Password is not correct")),
        ErrorMessage::PleaseTryAgainLater =>
            ServiceError::BadRequest(String::from("Please, try again later")),
        ErrorMessage::RevisionAlreadyActiveOrDeleted =>
            ServiceError::BadRequest(String::from("Revision already active or deleted")),
        ErrorMessage::RoleNotFound =>
            ServiceError::BadRequest(String::from("Role not found")),
        ErrorMessage::SelectedFileIsNotImage =>
            ServiceError::BadRequest(String::from("Selected file is not image")),
        ErrorMessage::SpecNotFound =>
            ServiceError::BadRequest(String::from("Spec not found")),
        ErrorMessage::CompanyIsNotSupplier =>
            ServiceError::BadRequest(String::from("The company is not supplier")),
        ErrorMessage::ComponentIsNotStandard =>
            ServiceError::BadRequest(String::from("The component is not standard")),
        ErrorMessage::DataHasAlready =>
            ServiceError::BadRequest(String::from("The data has already")),
        ErrorMessage::FileDoedNotSupportVersioning =>
            ServiceError::BadRequest(String::from("The file does not support versioning")),
        ErrorMessage::UserHasAlreadyMemberInTheCompany =>
            ServiceError::BadRequest(String::from("The user has already member in the company")),
        ErrorMessage::UserNotFoundInCompany =>
            ServiceError::BadRequest(String::from("The user not found in the company")),
        ErrorMessage::LicenseAlreadySetForComponent =>
            ServiceError::BadRequest(String::from("This license for the component is already")),
        ErrorMessage::DoesNotWorkForBaseComponent =>
            ServiceError::BadRequest(String::from("This not work for base component")),
        ErrorMessage::StanardIsAlreadyAssociatedWithComponent =>
            ServiceError::BadRequest(String::from("This standard is already associated with the component")),
        ErrorMessage::UsernameIsAlreadyUsed =>
            ServiceError::BadRequest(String::from("This username is already used")),
        ErrorMessage::TokenNotFound =>
            ServiceError::BadRequest(String::from("Token not found")),
        ErrorMessage::UnsuccessfulCheckData =>
            ServiceError::BadRequest(String::from("Unsuccessful check data")),
        ErrorMessage::NeedToChooseCompanyOrRepresentative =>
            ServiceError::BadRequest(String::from("You need to choose a company or a representative company")),
        ErrorMessage::TokenIsInvalid =>
            ServiceError::BadRequest(String::from("Your token is invalid")),
        ErrorMessage::NameAlreadyThereX(name, x) =>
            ServiceError::BadRequest(format!("This {} name is already there. Id: {}", name, x)),
        ErrorMessage::IdsAlreadyHas(error_ids) =>
            ServiceError::BadRequest(format!("This ids {:?} already has", error_ids)),
    }
}