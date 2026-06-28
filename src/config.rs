use uuid::Uuid;
use structopt::StructOpt;
use crate::cli_args::Opt;

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