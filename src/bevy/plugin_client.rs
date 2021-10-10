use crate::shared::{ConnectionHandle, Enveloppe, GenericParser, NetworkEvent};
use crate::client::Client;
use bevy::prelude::*;
use log::{trace, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default, Debug)]
pub struct WebSocketClient {}

impl Plugin for WebSocketClient {
    fn build(&self, app: &mut AppBuilder) {
        let client = Client::new();
        let router = Arc::new(Mutex::new(GenericParser::new()));
        let map = HashMap::<String, Vec<(ConnectionHandle, Enveloppe)>>::new();
        let network_events = Vec::<NetworkEvent>::new();
        app.insert_resource(client)
            .insert_resource(router)
            .insert_resource(map)
            .insert_resource(network_events)
            .add_event::<NetworkEvent>()
            .add_stage_before(CoreStage::First, "network", SystemStage::single_threaded())
            .add_system_to_stage("network", consume_messages.system())
            .add_system_to_stage("network", super::shared::handle_network_events.system());
    }
}

fn consume_messages(
    client: Res<Client>,
    mut hmap: ResMut<HashMap<String, Vec<(ConnectionHandle, Enveloppe)>>>,
    mut network_events: ResMut<Vec<NetworkEvent>>,
) {
    if !client.is_running() {
        return;
    }

    while let Some(ev) = client.try_recv() {
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
                trace!("received network event: {:?}", other);
                network_events.push(other);
            }
        }
    }
}

