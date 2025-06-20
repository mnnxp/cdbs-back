use async_graphql::MergedObject;

pub(crate) mod handler;

mod company;
mod component;
mod relate;
mod standard;
mod supplier_service;
mod user;

pub use company::{CompanyMutation, CompanyQuery};
pub use component::{ComponentMutation, ComponentQuery, component_model};
pub use standard::{StandardMutation, StandardQuery, standard_model};
pub use supplier_service::{ServiceMutation, ServiceQuery, service_model};
pub use user::{UserMutation, UserQuery};
pub use relate::extension::ExtensionMutation;
pub use relate::discussion::{DiscussionQuery, DiscussionMutation, discussion_model};
pub use relate::file;
pub use relate::keyword::{KeywordMutation, KeywordQuery};
pub use relate::language::LanguageQuery;
pub use relate::license::{LicenseMutation, LicenseQuery};
pub use relate::param::{ParamMutation, ParamQuery};
pub use relate::program::{ProgramMutation, ProgramQuery};
pub use relate::region::{RegionMutation, RegionQuery};
pub use relate::spec::SpecQuery;
pub use relate::storage::{StorageMutation, StorageQuery};
pub use relate::type_access::TypeAccessQuery;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    UserQuery,
    ComponentQuery,
    StandardQuery,
    ServiceQuery,
    CompanyQuery,
    // ExtensionQuery,
    DiscussionQuery,
    KeywordQuery,
    LanguageQuery,
    LicenseQuery,
    ParamQuery,
    ProgramQuery,
    RegionQuery,
    SpecQuery,
    StorageQuery,
    TypeAccessQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    UserMutation,
    ComponentMutation,
    StandardMutation,
    ServiceMutation,
    CompanyMutation,
    ExtensionMutation,
    DiscussionMutation,
    KeywordMutation,
    // LanguageMutation,
    LicenseMutation,
    ParamMutation,
    ProgramMutation,
    RegionMutation,
    // SpecMutation,
    StorageMutation,
    // TypeAccessMutation,
);

use crate::graphql::handler::{graphiql, graphql};
use actix_web::{guard, web};

pub(super) fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").guard(guard::Post()).to(graphql))
        .service(web::resource("/").guard(guard::Get()).to(graphiql));
}
