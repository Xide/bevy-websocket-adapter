mod handle;
mod router;
mod event;

pub use handle::ConnectionHandle;
pub use router::*;
pub use event::*;

#[macro_export]
macro_rules! impl_message_type {
    ( $type:ty, $name:expr ) => {
        impl bevy_websocket_adapter::shared::MessageType for $type {
            fn message_type() -> &'static str {
                $name
            }
        }
    };
}
