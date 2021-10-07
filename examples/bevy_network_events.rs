extern crate bevy_websocket;
use log::info;
use ::bevy::prelude::*;
use bevy_websocket::{
    server::{Server, NetworkEvent},
    bevy::{WebSocketServer}
};


fn start_listen(
    mut ws: ResMut<Server>
) {
    ws.listen("0.0.0.0:12345").expect("failed to start websocket server");
}

fn listen_for_events(
    mut evs: EventReader<NetworkEvent>
) {
    for ev in evs.iter() {
        info!("received NetworkEvent : {:?}", ev);
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(WebSocketServer::default())
        .add_startup_system(start_listen.system())
        .add_system(listen_for_events.system())
        .run();
}
