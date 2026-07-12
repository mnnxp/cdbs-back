use crate::cli_args::Opt;
use structopt::StructOpt;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref CONFIG: Opt = Opt::from_args();
}

/// Root component UUID (self-referencing parent)
pub(crate) fn root_component_uuid() -> Uuid {
    CONFIG.root_component_uuid
}

/// Root standard UUID (self-referencing parent)
pub(crate) fn root_standard_uuid() -> Uuid {
    CONFIG.root_standard_uuid
}

/// Root modification UUID (self-referencing parent)
pub(crate) fn root_modification_uuid() -> Uuid {
    CONFIG.root_modification_uuid
}

/// Default image UUID (placeholder)
pub(crate) fn default_image_uuid() -> Uuid {
    CONFIG.default_image_uuid
}

/// Default user UUID (anonymous)
pub(crate) fn default_user_uuid() -> Uuid {
    CONFIG.default_user_uuid
}
