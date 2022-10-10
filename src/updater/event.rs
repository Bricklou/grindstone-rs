#[derive(Debug)]
pub struct Progress {
    pub current: u32,
    pub max: u32,
    pub message: String,
}

#[derive(Debug)]
pub enum EventType {
    Starting,
    CreatingFolders,
    DownloadManifest,
    SearchingForJRE,
    DownloadJRE(Progress),
}

#[derive(Debug)]
pub struct CallbackEvent {
    pub event_type: EventType,
    pub message: String,
}

pub type CallbackFn = fn(CallbackEvent);

#[macro_export]
macro_rules! invoke_callback {
    ($s:expr, $t:expr,$m:expr) => {{
        use $crate::updater::event::CallbackEvent;
        log::info!("{}", $m);
        $s.config.event_callback.as_ref()(CallbackEvent {
            event_type: $t,
            message: String::from($m),
        });
    }};
}
