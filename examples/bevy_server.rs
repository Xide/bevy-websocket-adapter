extern crate bevy_websocket_adapter;
use ::bevy::prelude::*;
use bevy_websocket_adapter::{
    bevy::{WebSocketServer, WsMessageInserter},
    impl_message_type,
    server::{ConnectionHandle, Server},
};
use log::info;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DummyEvent {
    a: u32,
}
impl_message_type!(DummyEvent, "dummy");

fn start_listen(mut ws: ResMut<Server>) {
    ws.listen("0.0.0.0:12345")
        .expect("failed to start websocket server");
}

fn listen_for_dummy(mut evs: EventReader<(ConnectionHandle, DummyEvent)>) {
    for (handle, ev) in evs.iter() {
        info!("received DummyEvent from {:?} : {:?}", handle, ev);
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(WebSocketServer::default())
        .add_startup_system(start_listen.system())
        .register_message_type::<DummyEvent>()
        .add_system(listen_for_dummy.system())
        .run();
}
