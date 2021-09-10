use async_graphql::{
    MergedObject,
    // MergedSubscription,
    // Object,
    // Schema,
    // SimpleObject,
    // Subscription
};

pub(crate) mod handler;

mod user;
mod component;
mod standard;
mod company;
mod relate;

pub use user::{UserQuery, UserMutation};
pub use component::{ComponentQuery, ComponentMutation};
pub use standard::{StandardQuery, StandardMutation};
pub use company::{CompanyQuery, CompanyMutation};
// pub use relate::extension::{ExtensionQuery, ExtensionMutation};
pub use relate::extension::ExtensionMutation;
pub use relate::keyword::{KeywordQuery, KeywordMutation};
// pub use relate::language::{LanguageQuery, LanguageMutation};
pub use relate::language::LanguageQuery;
pub use relate::license::{LicenseQuery, LicenseMutation};
pub use relate::param::{ParamQuery, ParamMutation};
pub use relate::program::{ProgramQuery, ProgramMutation};
// pub use relate::spec::{SpecQuery, SpecMutation};
pub use relate::spec::SpecQuery;
pub use relate::storage::{StorageQuery, StorageMutation};

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

use actix_web::{guard, web};
use crate::graphql::handler::{graphql, graphiql};

pub(super) fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").guard(guard::Post()).to(graphql))
        .service(web::resource("/").guard(guard::Get()).to(graphiql));
}
