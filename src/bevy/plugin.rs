use crate::server::MessageType;
use crate::server::{ConnectionHandle, Enveloppe, GenericParser, NetworkEvent, Server};
use bevy::prelude::*;
use log::warn;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default, Debug)]
pub struct WebSocketServer {}

impl Plugin for WebSocketServer {
    fn build(&self, app: &mut AppBuilder) {
        let server = Server::new();
        let router = Arc::new(Mutex::new(GenericParser::new()));
        let map = HashMap::<String, Vec<(ConnectionHandle, Enveloppe)>>::new();
        let network_events = Vec::<NetworkEvent>::new();
        app.insert_resource(server)
            .insert_resource(router)
            .insert_resource(map)
            .insert_resource(network_events)
            .add_event::<NetworkEvent>()
            .add_stage_before(CoreStage::First, "network", SystemStage::single_threaded())
            .add_system_to_stage("network", consume_messages.system())
            .add_system_to_stage("network", handle_network_events.system());
    }
}

fn consume_messages(
    server: Res<Server>,
    mut hmap: ResMut<HashMap<String, Vec<(ConnectionHandle, Enveloppe)>>>,
    mut network_events: ResMut<Vec<NetworkEvent>>,
) {
    if !server.is_running() {
        return;
    }

    while let Some(ev) = server.recv() {
        match ev {
            NetworkEvent::Message(handle, raw_ev) => {
                trace!("consuming message from {:?}", handle);
                if let Ok(enveloppe) = serde_json::from_reader::<std::io::Cursor<Vec<u8>>, Enveloppe>(
                    std::io::Cursor::new(raw_ev),
                ) {
                    let tp = enveloppe.message_type.to_string();
                    let mut v = if let Some(x) = hmap.remove(&tp) {
                        x
                    } else {
                        Vec::new()
                    };
                    v.push((handle, enveloppe.clone()));
                    hmap.insert(tp, v);
                } else {
                    warn!("failed to deserialize message from {:?}", handle);
                    continue;
                }
            }
            other => {
                error!("received network event: {:?}", other);
                network_events.push(other);
            }
        }
    }
}

fn handle_network_events(
    mut events: ResMut<Vec<NetworkEvent>>,
    mut sink: EventWriter<NetworkEvent>,
) {
    for ev in events.drain(..) {
        sink.send(ev);
    }
}

fn add_message_consumer<T>(
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
    fn register_message_type<T>(&mut self) -> &mut Self
    where
        T: MessageType + 'static;
}

impl WsMessageInserter for AppBuilder {
    fn register_message_type<T>(&mut self) -> &mut Self
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
