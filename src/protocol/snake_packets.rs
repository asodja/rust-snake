use game::Snake;
use protocol::Packet;
use game::snake::StateChange::Spawn;
use game::snake::StateChange::PositionChange;
use game::snake::StateChange::AngleChange;
use game::snake::StateChange::SpeedChange;
use game::snake::StateChange::WantedAngleChange;
use extension::number_extensions::NumberExtension;
use std::f32::consts::PI;
use game::snake::StateChange::Death;

impl Snake {
    pub fn get_packets(&self) -> Vec<Packet> {
        let mut packets = Vec::new();
        if self.updates.contains(&Death) {
            packets.push(self.get_remove_packet());
            return packets;
        }

        if self.updates.contains(&WantedAngleChange)
            || self.updates.contains(&AngleChange)
            || self.updates.contains(&SpeedChange) {
            packets.push(self.get_movement_packet());
        }

        if self.updates.contains(&PositionChange) || self.updates.contains(&Spawn) {
            packets.push(self.get_position_packet());
        }

        return packets;
    }

    fn get_remove_packet(&self) -> Packet {
        Packet::new()
            .write_char('s')
            .write_u16(self.id)
    }

    fn get_position_packet(&self) -> Packet {
        // TODO send parts only the first time
        // after that only head position can be sent
        Packet::new()
            .write_char('g')
            .write_u16(self.id)
            .write_f16(self.head().x)
            .write_f16(self.head().y)
            .write_f16(self.tail().x)
            .write_f16(self.tail().y)
    }

    fn get_movement_packet(&self) -> Packet {
        let mut packet = Packet::new()
            .write_char(self.get_movement_type())
            .write_u16(self.id);
        if self.updates.contains(&AngleChange) {
            packet = packet.write_u8((self.angle / (2.0 * PI) * 256.0) as u8);
        }
        if self.updates.contains(&WantedAngleChange) {
            packet = packet.write_u8((self.wanted_angle / (2.0 * PI) * 256.0) as u8);
        }
        if self.updates.contains(&SpeedChange) {
            packet = packet.write_u8((self.speed / 18.0) as u8);
        }
        return packet;
    }

    fn get_movement_type(&self) -> char {
        // Current implemented client actually does not care about
        // rotation direction, but could come handy if we would improve it
        return if !self.updates.contains(&WantedAngleChange) {
            if !self.updates.contains(&AngleChange) {
                '3'
            } else {
                'e'
            }
        } else if !self.updates.contains(&AngleChange) {
            'E'
        } else if !self.updates.contains(&SpeedChange) {
            if (self.wanted_angle - self.angle).is_clockwise() {
                '5'
            } else {
                '3'
            }
        } else {
            if (self.wanted_angle - self.angle).is_clockwise() {
                '4'
            } else {
                'e'
            }
        }
    }
}
