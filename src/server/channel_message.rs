use ws::Sender;
use protocol::client_command::ClientCommand;

pub enum ChannelMessage {
    SocketOpen(Sender),
    ClientData(u32, ClientCommand),
    WorldUpdate
}