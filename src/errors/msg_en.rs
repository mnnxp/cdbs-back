use super::err_msg::ErrorMessage;

pub(crate) fn value_in_err_msg_en(err_msg: ErrorMessage) -> String {
    match err_msg {
        ErrorMessage::AccessDenied => String::from("Access denied"),
        ErrorMessage::AccessNotAdded => String::from("Access has not been added"),
        ErrorMessage::AccessNotFoundCompany => String::from("Access not found for company"),
        ErrorMessage::AccessNotFoundUser => String::from("Access not found for user"),
        ErrorMessage::BadFilename => String::from("Bad filename"),
        ErrorMessage::CannotDeleteRows => String::from("Cannot delete rows"),
        ErrorMessage::DataNotFound => String::from("Data not found"),
        ErrorMessage::FoundDuplicateData => String::from("Found duplicate data"),
        ErrorMessage::FoundDuplicateToken => String::from("Found duplicate token"),
        ErrorMessage::DuplicateOfExistingData => {
            String::from("Duplication of existing data was detected")
        }
        ErrorMessage::ErrorDeleteRole => String::from("Error when deleting a role"),
        ErrorMessage::ErrorIncorrectId => String::from("Error incorrect id"),
        ErrorMessage::ErrorRecordingDependencyData => {
            String::from("Error recording dependency data")
        }
        ErrorMessage::FailGetUserData => String::from("Fail get SlimUser from Claims"),
        ErrorMessage::FailedAddAccess => String::from("Failed add access for target company"),
        ErrorMessage::FailedCheckData => String::from("Failed check data"),
        ErrorMessage::FailedCheckRole => String::from("Failed check role data"),
        ErrorMessage::FailedCheckSpec => String::from("Failed check spec for standard"),
        ErrorMessage::FailedDeleteAccessForCompany => {
            String::from("Failed delete access for target company")
        }
        ErrorMessage::FailedDeleteSupplierComponent => {
            String::from("Failed delete company of suppliers component")
        }
        ErrorMessage::FailedDeleteRelatedStandardsComponent => {
            String::from("Failed delete related standards to component")
        }
        ErrorMessage::FailedDeleteSuppliersComponent => {
            String::from("Failed delete related suppliers to component")
        }
        ErrorMessage::FailedGetCompaniesWithAccessComponent => {
            String::from("Failed get companies list have access to component")
        }
        ErrorMessage::FailedGetCompaniesWithAccessStandard => {
            String::from("Failed get companies list have access to standard")
        }
        ErrorMessage::FailedGetCompaniesWithAccessService => {
            String::from("Failed get companies list have access to service")
        }
        ErrorMessage::FailedMatchArguments => String::from("Failed match arguments"),
        ErrorMessage::FailedSetAccessCompany => {
            String::from("Failed set access for target company")
        }
        ErrorMessage::FailedUpdateData => String::from("Failed update data"),
        ErrorMessage::FailedUpdateRoleMember => String::from("Failed update role member"),
        ErrorMessage::FailedUpdateServiceBadStatus => {
            String::from("Failed update service, inappropriate status")
        }
        ErrorMessage::FailedWriteMetadata => String::from("Failed write metadata"),
        ErrorMessage::FailedRemoveAccessForRole => String::from("Failed: access not delete"),
        ErrorMessage::FileObjectNotFound => String::from("File to object association not found"),
        ErrorMessage::TextMustLess(max_bit) => format!(
            "Text must be less than {} bit (~{} symbols)",
            max_bit,
            max_bit / 2
        ),
        ErrorMessage::NeedSetUuidOrUsername => String::from("Need set userUuid or username"),
        ErrorMessage::NoActiveFileRevisionFound => String::from("No active file revision found"),
        ErrorMessage::NoSuitableSupplierHasBeenFound => {
            String::from("No suitable supplier has been found")
        }
        ErrorMessage::NotFoundAccessForRole => String::from("Not found access for target role"),
        ErrorMessage::NotFoundFilename => String::from("Not found filename"),
        ErrorMessage::NotFoundFilesetData => String::from("Not found fileset data"),
        ErrorMessage::NotFoundKeywords => String::from("Not found keywords"),
        ErrorMessage::NotFoundModificationData => String::from("Not found modification data"),
        ErrorMessage::NotFoundParamsForAddingOrUpdaing => {
            String::from("Not found params for adding or updating")
        }
        ErrorMessage::NotFoundParamsForDeleting => String::from("Not found params for deleting"),
        ErrorMessage::NotFoundRepresentative => String::from("Not found representative"),
        ErrorMessage::NotFoundRolesForCompany => String::from("Not found roles for company"),
        ErrorMessage::NotFoundSpecs => String::from("Not found specs"),
        ErrorMessage::NotFoundStandard => String::from("Not found standard"),
        ErrorMessage::NotFoundService => String::from("Not found service"),
        ErrorMessage::NotFoundTargetFile => String::from("Not found target file"),
        ErrorMessage::NotMorePathInOneQuery => String::from("Not more 100 path in one query"),
        ErrorMessage::NotFoundDiscussion => String::from("Not found discussion"),
        ErrorMessage::PasswordIsNotCorrect => String::from("Password is not correct"),
        ErrorMessage::PleaseTryAgainLater => String::from("Please, try again later"),
        ErrorMessage::RevisionAlreadyActiveOrDeleted => {
            String::from("Revision already active or deleted")
        }
        ErrorMessage::RoleNotFound => String::from("Role not found"),
        ErrorMessage::SelectedFileIsNotImage => String::from("Selected file is not image"),
        ErrorMessage::SpecNotFound => String::from("Spec not found"),
        ErrorMessage::CompanyIsNotSupplier => String::from("The company is not supplier"),
        ErrorMessage::ComponentIsNotStandard => String::from("The component is not standard"),
        ErrorMessage::DataHasAlready => String::from("The data has already"),
        ErrorMessage::FileDoedNotSupportVersioning => {
            String::from("The file does not support versioning")
        }
        ErrorMessage::UserHasAlreadyMemberInTheCompany => {
            String::from("The user has already member in the company")
        }
        ErrorMessage::UserNotFoundInCompany => String::from("The user not found in the company"),
        ErrorMessage::LicenseAlreadySetForComponent => {
            String::from("This license for the component is already")
        }
        ErrorMessage::DoesNotWorkForBaseComponent => {
            String::from("This not work for base component")
        }
        ErrorMessage::StanardIsAlreadyAssociatedWithComponent => {
            String::from("This standard is already associated with the component")
        }
        ErrorMessage::UsernameIsAlreadyUsed => String::from("This username is already used"),
        ErrorMessage::TokenNotFound => String::from("Token not found"),
        ErrorMessage::UnsuccessfulCheckData => String::from("Unsuccessful check data"),
        ErrorMessage::NeedToChooseCompanyOrRepresentative => {
            String::from("You need to choose a company or a representative company")
        }
        ErrorMessage::TokenIsInvalid => String::from("Your token is invalid"),
        ErrorMessage::NameAlreadyThereX(name, x) => {
            format!("This {} name is already there. Id: {}", name, x)
        }
        ErrorMessage::IdsAlreadyHas(error_ids) => format!("This ids {:?} already has", error_ids),
    }
}
