
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ClientCommand {
    ClientInit,
    Ping,
    AngleChange(u8),
    SpeedChange(bool),
    ClientExit,
    InvalidCommand
}

pub fn decode_command(vec: Vec<u8>) -> ClientCommand {
    if vec.is_empty() {
        return ClientCommand::InvalidCommand;
    }
    if vec.len() > 2 && *vec.get(0).unwrap() == 115 {
        // TODO Decode init, skin id byte 1, rest name
        return ClientCommand::ClientInit;
    }
    return match vec.get(0).unwrap() {
        x @ 0...250 => ClientCommand::AngleChange(*x),
        251 => ClientCommand::Ping,
        x @ 253...254 => ClientCommand::SpeedChange(*x == 253),
        255 => ClientCommand::ClientExit,
        _ => ClientCommand::InvalidCommand
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_invalid_command_if_empty_vector() {
        // Given, When
        let actual_command = decode_command(vec![]);

        // Then
        assert_eq!(actual_command, ClientCommand::InvalidCommand);
    }

    #[test]
    fn should_decode_angle_command() {
        // Given, When
        let actual_command = decode_command(vec![125]);

        // Then
        assert_eq!(actual_command, ClientCommand::AngleChange(125));
    }

    #[test]
    fn should_decode_ping_command() {
        // Given, When
        let actual_command = decode_command(vec![251]);

        // Then
        assert_eq!(actual_command, ClientCommand::Ping);
    }

    #[test]
    fn should_decode_speed_command() {
        // Given, When
        let actual_command = decode_command(vec![253]);

        // Then
        assert_eq!(actual_command, ClientCommand::SpeedChange(true));
    }

    #[test]
    fn should_decode_client_exit() {
        // Given, When
        let actual_command = decode_command(vec![255]);

        // Then
        assert_eq!(actual_command, ClientCommand::ClientExit);
    }

}