use grindstone::GrindstoneUpdater;

extern crate env_logger;
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    log::info!("Hello !");

    let mut updater = GrindstoneUpdater::new();

    updater.update().await?;

    Ok(())
}
