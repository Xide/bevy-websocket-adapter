extern crate bevy_websocket_adapter;
use ::bevy::prelude::*;
use bevy_websocket_adapter::{
    bevy::{WebSocketServer, WsMessageInserter},
    server::{ConnectionHandle, Server},
};
use log::info;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Ping {}
#[derive(Serialize, Deserialize, Debug)]
struct Pong {}

fn start_listen(mut ws: ResMut<Server>) {
    ws.listen("0.0.0.0:12345")
        .expect("failed to start websocket server");
}

fn respond_to_pings(mut evs: EventReader<(ConnectionHandle, Ping)>, srv: Res<Server>) {
    for (handle, ev) in evs.iter() {
        info!("received ping from {:?} : {:?}", handle, ev);
        let payload = serde_json::to_vec(&Pong {}).unwrap();
        srv.send_message(
            handle,
            tokio_tungstenite::tungstenite::Message::Binary(payload),
        )
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(WebSocketServer::default())
        .add_startup_system(start_listen.system())
        .register_message_type::<Ping>("ping")
        .add_system(respond_to_pings.system())
        .run();
}
