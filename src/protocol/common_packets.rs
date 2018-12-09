use protocol::Packet;
use game::config::PROTOCOL_VERSION;
use game::config::WORLD_WIDTH;
use game::config::WORLD_HEIGHT;

pub fn init_packet(game_id: u16) -> Packet {
    Packet::new()
        .write_char('a')
        .write_u16(WORLD_WIDTH as u16)
        .write_u16(WORLD_HEIGHT as u16)
        .write_u8(PROTOCOL_VERSION as u8)
        .write_u16(game_id)
}

pub fn ping_packet() -> Packet {
    Packet::new().write_char('p')
}