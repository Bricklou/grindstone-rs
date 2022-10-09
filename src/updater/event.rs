use crate::GrindstoneUpdater;

#[derive(Debug)]
pub enum EventType {
    Starting,
    CreatingFolders,
}

#[derive(Debug)]
pub struct CallbackEvent {
    pub event_type: EventType,
    pub message: String,
}

pub type CallbackFn = fn(CallbackEvent);

impl GrindstoneUpdater {
    pub fn invoke_callback<S: Into<String>>(&self, event_type: EventType, message: S) {
        let message = message.into();

        log::info!("{}", message);
        self.config.event_callback.as_ref()(CallbackEvent {
            event_type,
            message,
        });
    }
}

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
