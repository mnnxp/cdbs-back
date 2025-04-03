use super::ServiceError;
use super::msg_en::value_in_err_msg_en;

/// Contains a list of all errors
pub(crate) enum ErrorMessage {
    /// "Access denied"
    AccessDenied,
    /// "Access has not been added"
    AccessNotAdded,
    /// "Access not found for company"
    AccessNotFoundCompany,
    /// "Access not found for user"
    AccessNotFoundUser,
    /// "Bad filename"
    BadFilename,
    /// "Cannot delete rows"
    CannotDeleteRows,
    /// "Data not found"
    DataNotFound,
    /// "Found duplicate data"
    FoundDuplicateData,
    /// "Found duplicate token"
    FoundDuplicateToken,
    /// "Duplication of existing data was detected"
    DuplicateOfExistingData,
    /// "Error when deleting a role"
    ErrorDeleteRole,
    /// "Error incorrect id"
    ErrorIncorrectId,
    /// "Error recording dependency data"
    ErrorRecordingDependencyData,
    /// "Fail get SlimUser from Claims"
    FailGetUserData,
    /// "Failed add access for target company"
    FailedAddAccess,
    /// "Failed check data"
    FailedCheckData,
    /// "Failed check role data"
    FailedCheckRole,
    /// "Failed check spec for standard"
    FailedCheckSpec,
    /// "Failed delete access for target company"
    FailedDeleteAccessForCompany,
    /// "Failed delete company of suppliers component"
    FailedDeleteSupplierComponent,
    /// "Failed delete related standards to component"
    FailedDeleteRelatedStandardsComponent,
    /// "Failed delete related suppliers to component"
    FailedDeleteSuppliersComponent,
    /// "Failed get companies list have access to component"
    FailedGetCompaniesWithAccessComponent,
    /// "Failed get companies list have access to standard"
    FailedGetCompaniesWithAccessStandard,
    /// "Failed get companies list have access to service"
    FailedGetCompaniesWithAccessService,
    /// "Failed match arguments"
    FailedMatchArguments,
    /// "Failed set access for target company"
    FailedSetAccessCompany,
    /// "Failed update data"
    FailedUpdateData,
    /// "Failed update role member"
    FailedUpdateRoleMember,
    /// "Failed update service"
    FailedUpdateServiceBadStatus,
    /// "Failed write metadata"
    FailedWriteMetadata,
    /// "Failed: access not delete"
    FailedRemoveAccessForRole,
    /// "File to object association not found"
    FileObjectNotFound,
    /// "Keywords must be less than 10 symbols"
    KeywordMustLess,
    /// "Need set userUuid or username"
    NeedSetUuidOrUsername,
    /// "No active file revision found"
    NoActiveFileRevisionFound,
    /// "No suitable supplier has been found"
    NoSuitableSupplierHasBeenFound,
    /// "Not found access for target role"
    NotFoundAccessForRole,
    /// "Not found filename"
    NotFoundFilename,
    /// "Not found fileset data"
    NotFoundFilesetData,
    /// "Not found keywords"
    NotFoundKeywords,
    /// "Not found modification data"
    NotFoundModificationData,
    /// "Not found params for adding or updating"
    NotFoundParamsForAddingOrUpdaing,
    /// "Not found params for deleting"
    NotFoundParamsForDeleting,
    /// "Not found representative"
    NotFoundRepresentative,
    /// "Not found roles for company"
    NotFoundRolesForCompany,
    /// "Not found specs"
    NotFoundSpecs,
    /// "Not found standard"
    NotFoundStandard,
    /// "Not found service"
    NotFoundService,
    /// "Not found target file"
    NotFoundTargetFile,
    /// "Not more 100 path in one query"
    NotMorePathInOneQuery,
    /// "Password is not correct"
    PasswordIsNotCorrect,
    /// "Please, try again later"
    PleaseTryAgainLater,
    /// "Revision already active or deleted"
    RevisionAlreadyActiveOrDeleted,
    /// "Role not found"
    RoleNotFound,
    /// "Selected file is not image"
    SelectedFileIsNotImage,
    /// "Spec not found"
    SpecNotFound,
    /// "The company is not supplier"
    CompanyIsNotSupplier,
    /// "The component is not standard"
    ComponentIsNotStandard,
    /// "The data has already"
    DataHasAlready,
    /// "The file does not support versioning"
    FileDoedNotSupportVersioning,
    /// "The user has already member in the company"
    UserHasAlreadyMemberInTheCompany,
    /// "The user not found in the company"
    UserNotFoundInCompany,
    /// "This license for the component is already"
    LicenseAlreadySetForComponent,
    /// "This not work for base component"
    DoesNotWorkForBaseComponent,
    /// "This standard is already associated with the component"
    StanardIsAlreadyAssociatedWithComponent,
    /// "This username is already used"
    UsernameIsAlreadyUsed,
    /// "Token not found"
    TokenNotFound,
    /// "Unsuccessful check data"
    UnsuccessfulCheckData,
    /// "You need to choose a company or a representative company"
    NeedToChooseCompanyOrRepresentative,
    /// "Your token is invalid"
    TokenIsInvalid,
    /// format!("This {} name is already there. Id: {}", name, x)
    NameAlreadyThereX(String, i32),
    /// format!("This ids {:?} already has", error_ids)
    IdsAlreadyHas(Vec<i32>),
}

pub(crate) fn get_err_msg(err_msg: ErrorMessage) -> ServiceError {
    ServiceError::BadRequest(
        format!("BadRequest: {0}", value_in_err_msg_en(err_msg))
    )
}