use ws::Sender;
use protocol::Packet;

pub trait WsSenderExtension {

    fn send_packets(&self, other: &Vec<Packet>);

}

impl WsSenderExtension for Sender {

    fn send_packets(&self, other: &Vec<Packet>) {
        for packet in other {
            self.send(packet.to_binary_msg()).ok();
        }
    }

}