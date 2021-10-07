extern crate bevy_websocket;
use log::info;
use ::bevy::prelude::*;
use bevy_websocket::{
    server::Server,
    bevy::{WebSocketServer, WsMessageInserter}
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DummyEvent {
    a: u32,
}


fn start_listen(
    mut ws: ResMut<Server>
) {
    ws.listen("0.0.0.0:12345").expect("failed to start websocket server");
}

fn listen_for_dummy(
    mut evs: EventReader<DummyEvent>
) {
    for ev in evs.iter() {
        info!("received DummyEvent : {:?}", ev);
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(WebSocketServer::default())
        .add_startup_system(start_listen.system())
        .register_message_type::<DummyEvent>("dummy")
        .add_system(listen_for_dummy.system())
        .run();
}
