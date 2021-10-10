extern crate bevy_websocket_adapter;
use ::bevy::prelude::*;
use bevy_websocket_adapter::{
    bevy::{WebSocketClient, WsMessageInserter},
    impl_message_type,
    client::Client,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DummyEvent {
    a: u32,
}
impl_message_type!(DummyEvent, "dummy");

fn connect_to_server(mut ws: ResMut<Client>) {
    ws.connect("ws://127.0.0.1:12345".to_string());
}

fn send_dummies(
    client: Res<Client>

) {
    client.send_message(&DummyEvent{a: 2});
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(WebSocketClient::default())
        .add_startup_system(connect_to_server.system())
        .add_message_type::<DummyEvent>()
        .add_system(send_dummies.system())
        .run();
}
