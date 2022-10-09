use grindstone::{config::ConfigBuilder, event::CallbackFn, GrindstoneUpdater};

extern crate env_logger;
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    log::info!("Hello !");

    let callback_fn: CallbackFn = |event| {
        println!("{:?} - {}", event.event_type, event.message);
    };

    let config = ConfigBuilder::default()
        .set_event_callback(Box::new(callback_fn))
        .minecraft_folder_path("./output")
        .name("My example version")
        .build()?;

    let mut updater = GrindstoneUpdater::new(config);

    updater.update().await?;

    Ok(())
}
