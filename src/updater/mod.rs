use log::info;

use crate::errors::GrindstoneResult;

pub struct GrindstoneUpdater {}

impl GrindstoneUpdater {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn update(&mut self) -> GrindstoneResult<()> {
        info!("Starting updater !");
        Ok(())
    }
}
