
use ws::Sender;
use server::ChannelMessage;
use ws::Handler;
use ws::Handshake;
use ws::Message;
use protocol::client_command::decode_command;
use ws::CloseCode;
use protocol::client_command::ClientCommand::ClientExit;
use protocol::client_command::ClientCommand::Ping;
use protocol::common_packets::ping_packet;
use server::ClientState;
use server::channel_message::ChannelMessage::SocketOpen;
use server::channel_message::ChannelMessage::ClientData;

pub struct Connection {
    pub ws: Sender,
    pub room_channel: crossbeam_channel::Sender<ChannelMessage>
}

impl Handler for Connection {

    fn on_open(&mut self, _shake: Handshake) -> ws::Result<()> {
        self.room_channel.send(SocketOpen(self.ws.clone()));
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let connection_id = self.ws.connection_id();
        let command = decode_command(msg.into_data());
        match command {
            Ping => {
                self.ws.send(ping_packet().to_binary_msg()).ok();
            },
            _ => {
                self.room_channel.send(ClientData(connection_id, command));
            }
        };
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, _reason: &str) {
        if code.ne(&ClientState::GAME_ROOM_CLOSE_CODE) {
            let connection_id = self.ws.connection_id();
            self.room_channel.send(ClientData(connection_id, ClientExit));
        }
    }

}