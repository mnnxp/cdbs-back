use async_graphql::MergedObject;

pub(crate) mod handler;

mod company;
mod component;
mod relate;
mod standard;
mod user;

pub use company::{CompanyMutation, CompanyQuery};
pub use component::{ComponentMutation, ComponentQuery};
pub use standard::{StandardMutation, StandardQuery};
pub use user::{UserMutation, UserQuery};
pub use relate::extension::ExtensionMutation;
pub use relate::keyword::{KeywordMutation, KeywordQuery};
pub use relate::language::LanguageQuery;
pub use relate::license::{LicenseMutation, LicenseQuery};
pub use relate::param::{ParamMutation, ParamQuery};
pub use relate::program::{ProgramMutation, ProgramQuery};
pub use relate::spec::SpecQuery;
pub use relate::storage::{StorageMutation, StorageQuery};

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    UserQuery,
    ComponentQuery,
    StandardQuery,
    CompanyQuery,
    // ExtensionQuery,
    KeywordQuery,
    LanguageQuery,
    LicenseQuery,
    ParamQuery,
    ProgramQuery,
    SpecQuery,
    StorageQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    UserMutation,
    ComponentMutation,
    StandardMutation,
    CompanyMutation,
    ExtensionMutation,
    KeywordMutation,
    // LanguageMutation,
    LicenseMutation,
    ParamMutation,
    ProgramMutation,
    // SpecMutation,
    StorageMutation,
);

use crate::graphql::handler::{graphiql, graphql};
use actix_web::{guard, web};

pub(super) fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").guard(guard::Post()).to(graphql))
        .service(web::resource("/").guard(guard::Get()).to(graphiql));
}
