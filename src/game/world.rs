
use std::collections::HashMap;
use game::Snake;
use protocol::client_command::ClientCommand::AngleChange;
use protocol::client_command::ClientCommand::SpeedChange;
use protocol::client_command::ClientCommand::ClientInit;
use protocol::client_command::ClientCommand::ClientExit;
use protocol::ClientCommand;
use game::Point;
use game::snake::StateChange::Death;
use game::config::WORLD_WIDTH;
use game::config::WORLD_HEIGHT;

#[allow(dead_code)]
pub struct World {
    pub snakes: HashMap<u16, Snake>,
}

impl Default for World {

    fn default() -> World {
        World {
            snakes: HashMap::new()
        }
    }

}

impl World {

    pub fn update(&mut self, delta_time: f32) {
        for snake in self.snakes.values_mut() {
            snake.update(delta_time);
        }

        for snake in self.snakes.values_mut() {
            snake.check_collision();
        }
    }

    pub fn execute_command(&mut self, snake_id: &u16, command: &ClientCommand) {
        match command {
            ClientInit => {
                self.spawn_snake(snake_id)
            },
            AngleChange(angle) => if let Some(snake) = self.snakes.get_mut(snake_id) {
                snake.change_angle(*angle)
            },
            SpeedChange(is_accelerating) => if let Some(snake) = self.snakes.get_mut(snake_id) {
                snake.change_acceleration(*is_accelerating)
            },
            ClientExit => if let Some(snake) = self.snakes.get_mut(snake_id) {
                snake.kill()
            },
            _ => {}
        }
    }

    fn spawn_snake(&mut self, snake_id: &u16) {
        let spawn_point = Point {
            x: WORLD_WIDTH as f32 / 2.0,
            y: WORLD_HEIGHT as f32 / 2.0,
        };
        let snake = Snake::new_spawned(snake_id, &spawn_point);
        self.snakes.insert(snake.id, snake);
    }

    pub fn clear_state(&mut self) {
        self.snakes.retain(|ref _id, ref snake| !snake.updates.contains(&Death));
        for snake in self.snakes.values_mut() {
            snake.updates.clear();
        }
    }

    pub fn get_ids_for_dead_clients(&self) -> Vec<u16> {
        self.snakes.values()
            .filter(|snake| snake.updates.contains(&Death))
            .map(|snake| snake.id)
            .collect()
    }

}