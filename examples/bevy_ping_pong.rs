extern crate bevy_websocket_adapter;
use ::bevy::prelude::*;
use bevy_websocket_adapter::{
    bevy::{WebSocketServer, WsMessageInserter},
    impl_message_type,
    server::{ConnectionHandle, Server},
};
use log::info;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Ping {}
impl_message_type!(Ping, "ping");

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Pong {}
impl_message_type!(Pong, "pong");

fn start_listen(mut ws: ResMut<Server>) {
    ws.listen("0.0.0.0:12345")
        .expect("failed to start websocket server");
}

fn respond_to_pings(mut evs: EventReader<(ConnectionHandle, Ping)>, srv: Res<Server>) {
    for (handle, ev) in evs.iter() {
        info!("received ping from {:?} : {:?}", handle, ev);
        srv.send_message(handle, &Pong {})
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(WebSocketServer::default())
        .add_startup_system(start_listen.system())
        .register_message_type::<Ping>()
        .add_system(respond_to_pings.system())
        .run();
}
