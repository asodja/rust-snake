use game::World;
use server::ClientState;
use crossbeam_channel::Receiver;
use server::ChannelMessage;
use std::time::SystemTime;
use std::time::Duration;
use crossbeam_channel::internal::channel;
use server::channel_message::ChannelMessage::SocketOpen;
use server::channel_message::ChannelMessage::ClientData;
use server::channel_message::ChannelMessage::WorldUpdate;
use ws::Sender;
use extension::time_extensions::TimeExtension;
use ws::CloseCode;
use extension::ws_sender_extension::WsSenderExtension;
use protocol::common_packets::init_packet;
use protocol::ClientCommand;
use game::config::WORLD_UPDATE_MS;

#[allow(dead_code)]
pub struct GameRoom {
    pub room_number: i32,
    pub client_state: ClientState,
    pub world: World
}

impl GameRoom {

    pub fn listen(&mut self, clients_msg_channel: Receiver<ChannelMessage>) {
        let mut last_update_time = SystemTime::now();
        let mut world_update_channel = channel::after(Duration::from_millis(0));
        loop {
            let message = select! {
                recv(clients_msg_channel, received) => received.unwrap()
                recv(world_update_channel, _) => WorldUpdate
            };
            match message {
                SocketOpen(ws) => self.on_open(ws),
                ClientData(conn_id, command) => self.on_message(conn_id, &command),
                WorldUpdate => {
                    let now = SystemTime::now();
                    let delta_time = now.millis_since(last_update_time);
                    self.update(delta_time);
                    last_update_time = now;
                    world_update_channel = channel::after(Duration::from_millis(WORLD_UPDATE_MS));
                }
            }
        }
    }

    fn on_open(&mut self, ws: Sender) {
        let connection_id = ws.connection_id();
        match self.client_state.try_add_client(ws) {
            Err((ws, _)) => ws.close(CloseCode::Error).expect("Ws close error"),
            Ok(game_id) => {
                let ws = self.client_state.connection_id_to_socket.get(&connection_id).expect("Socket to be added");
                if let Err(_) = ws.send( init_packet(game_id).to_binary_msg()) {
                    ws.close(CloseCode::Error).expect("Error sending init packet");
                }
            }
        };
    }

    fn on_message(&mut self, conn_id: u32, command: &ClientCommand) {
        if let Some(game_id) = self.client_state.connection_id_to_game_id.get(&conn_id) {
            self.world.execute_command(game_id, command);
        }
    }

    fn update(&mut self, delta_time: f32) {
        // Update and send updates
        self.world.update(delta_time);
        self.send_packets();

        // Clear dead snakes and disconnect clients
        let dead_clients = self.world.get_ids_for_dead_clients();
        dead_clients.iter().for_each(|id| {
            self.client_state.disconnect_and_remove_client(*id)
        });

        // Clear World state before next update
        self.world.clear_state();
    }

    fn send_packets(&self) {
        let mut packets = Vec::with_capacity(100);
        for snake in self.world.snakes.values() {
            packets.extend(snake.get_packets());
        }

        for snake in self.world.snakes.values() {
            if let Some(ws) = &self.client_state.get_sender_with_game_id(snake.id) {
                ws.send_packets(&packets);
            }
        }
    }

}