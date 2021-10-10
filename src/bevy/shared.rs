use crate::shared::{ConnectionHandle, Enveloppe, GenericParser, MessageType, NetworkEvent};
use bevy::prelude::*;
use log::warn;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) fn handle_network_events(
    mut events: ResMut<Vec<NetworkEvent>>,
    mut sink: EventWriter<NetworkEvent>,
) {
    for ev in events.drain(..) {
        sink.send(ev);
    }
}

pub(crate) fn add_message_consumer<T>(
    key: Local<String>,
    mut hmap: ResMut<HashMap<String, Vec<(ConnectionHandle, Enveloppe)>>>,
    router: Res<Arc<Mutex<GenericParser>>>,
    mut queue: EventWriter<(ConnectionHandle, T)>,
) where
    T: Send + Sync + 'static,
{
    if let Some(values) = hmap.remove(&*key) {
        for (handle, v) in values {
            let enveloppe = router.lock().unwrap().parse_enveloppe(&v);
            match enveloppe {
                Ok(dat) => match GenericParser::try_into_concrete_type::<T>(dat) {
                    Ok(msg) => {
                        queue.send((handle, msg));
                    }
                    Err(e) => {
                        warn!("failed to downcast : {}", e);
                    }
                },
                Err(e) => {
                    warn!("failed to parse type enveloppe : {}", e);
                    continue;
                }
            };
        }
    }
}

pub trait WsMessageInserter {
    #[deprecated(
        since = "0.1.4",
        note = "Use [`add_message_type`](#method.add_message_type) instead."
    )]
    fn register_message_type<T>(&mut self) -> &mut Self
    where
        T: MessageType + 'static,
    {
        self.add_message_type::<T>()
    }
    fn add_message_type<T>(&mut self) -> &mut Self
    where
        T: MessageType + 'static;
}

impl WsMessageInserter for AppBuilder {
    fn add_message_type<T>(&mut self) -> &mut Self
    where
        T: MessageType + 'static,
    {
        self.add_event::<(ConnectionHandle, T)>();
        let router = self
            .app
            .world
            .get_resource::<Arc<Mutex<GenericParser>>>()
            .expect("cannot register message before WebSocketServer initialization");
        router.lock().unwrap().insert_type::<T>();

        self.add_system(add_message_consumer::<T>.system().config(|params| {
            params.0 = Some(T::message_type().to_string());
        }));
        self
    }
}
