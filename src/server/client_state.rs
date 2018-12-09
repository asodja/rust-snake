
use std::collections::HashMap;
use ws::Sender;
use std::collections::HashSet;
use ws::CloseCode;

#[allow(dead_code)]
pub struct ClientState {
    pub game_ids: HashSet<u16>,
    pub connection_id_to_socket: HashMap<u32, Sender>,
    pub game_id_to_connection_id: HashMap<u16, u32>,
    pub connection_id_to_game_id: HashMap<u32, u16>,
    pub last_game_id: u16
}

impl Default for ClientState {
    fn default() -> ClientState {
        ClientState {
            game_ids: HashSet::new(),
            connection_id_to_socket: HashMap::new(),
            game_id_to_connection_id: HashMap::new(),
            connection_id_to_game_id: HashMap::new(),
            last_game_id: 0
        }
    }
}

impl ClientState {

    pub const GAME_ROOM_CLOSE_CODE: CloseCode = CloseCode::Other(4000);

    pub fn try_add_client(&mut self, ws: Sender) -> Result<u16, (Sender, &str)> {
        if self.connection_id_to_socket.contains_key(&ws.connection_id()) {
            return Err((ws, "Duplicate connection id"));
        }

        match self.get_game_id() {
            Ok(id) => {
                self.last_game_id = id;
                self.game_ids.insert(id);
                self.game_id_to_connection_id.insert(id, ws.connection_id());
                self.connection_id_to_game_id.insert(ws.connection_id(), id);
                self.connection_id_to_socket.insert(ws.connection_id(), ws);
                Ok(id)
            },
            Err(msg) => Err((ws, msg))
        }
    }

    pub fn disconnect_and_remove_client(&mut self, game_id: u16) {
        if let Some(ws) = self.get_sender_with_game_id(game_id) {
            ws.close(ClientState::GAME_ROOM_CLOSE_CODE).ok();
        }
        if let Some(connection_id) = self.game_id_to_connection_id.get(&game_id) {
            self.connection_id_to_game_id.remove(connection_id);
            self.connection_id_to_socket.remove(connection_id);
        }
        self.game_id_to_connection_id.remove(&game_id);
        self.game_ids.remove(&game_id);
    }

    fn get_game_id(&self) -> Result<u16, &'static str> {
        for i in 1..u16::max_value() {
            let id = self.last_game_id.wrapping_add(i);
            if !self.game_ids.contains(&id) {
                return Ok(id)
            }
        }
        return Err("No id found")
    }

    pub fn get_sender_with_game_id(&self, game_id: u16) -> Option<&Sender> {
        match self.game_id_to_connection_id.get(&game_id) {
            Some(connection_id) => self.connection_id_to_socket.get(&connection_id),
            None => Option::None
        }
    }

}