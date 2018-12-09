extern crate ws;
#[macro_use]
extern crate crossbeam_channel;

use crossbeam_channel as channel;
use std::thread;
use game::World;
use server::ChannelMessage;
use server::GameRoom;
use server::Connection;
use server::ClientState;
use ws::Builder;
use ws::Settings;

pub mod server;
pub mod game;
pub mod extension;
pub mod protocol;

fn main() {
    let (sender, receiver) = channel::unbounded::<ChannelMessage>();

    thread::spawn(move || {
        GameRoom {
            room_number: 0,
            client_state: ClientState::default(),
            world: World::default()
        }.listen(receiver);
    });

    let settings = Settings {
        max_connections: 5000,
        tcp_nodelay: true,
        ..Settings::default()
    };

    let socket = Builder::new()
        .with_settings(settings)
        .build(move |ws| {
            Connection { ws, room_channel: sender.clone() }
        }).expect("Panicking on WebSocket build!");

    // Change address to 0.0.0.0:<port> when you
    // start accepting clients outside of localhost
    socket.listen("localhost:8081").expect("Panicking on WebSocket listen!");
}
