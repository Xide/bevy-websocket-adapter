mod router;
mod ws_handler;

pub use router::*;
pub use ws_handler::*;

#[macro_export]
macro_rules! impl_message_type {
    ( $type:ty, $name:expr ) => {
        impl bevy_websocket_adapter::server::MessageType for $type {
            fn message_type() -> &'static str {
                $name
            }
        }
    };
}
