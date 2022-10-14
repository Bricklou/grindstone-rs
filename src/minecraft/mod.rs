pub mod java;
mod vanilla;

pub use vanilla::models::version_data::{
    asset_index_info::AssetIndexInfo, library::Library, VersionData,
};
pub use vanilla::models::version_manifest::*;
