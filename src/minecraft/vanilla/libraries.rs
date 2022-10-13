use crate::{errors::GrindstoneResult, minecraft::VersionData, GrindstoneUpdater};

use super::models::version_data::library::Library;

impl Library {
    pub async fn install_libraries(version_data: VersionData) -> GrindstoneResult<()> {
        let needed_libraries = version_data.needed_libraries();

        //let downloads = needed_libraries.iter().map(|l|)

        Ok(())
    }
}
