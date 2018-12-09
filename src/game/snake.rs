use std::collections::VecDeque;
use game::Point;
use game::config::NORMAL_SPEED_PIXELS_PER_SECOND;
use game::config::ACCELERATED_SPEED_PIXELS_PER_SECOND;
use game::config::ACCELERATION_PER_MILLISECOND;
use extension::number_extensions::NumberExtension;
use game::config::SNAKE_ANGULAR_SPEED_PER_MS;
use std::f32::consts::PI;
use game::config::HALF_MAX_ANGLE;
use game::snake::StateChange::WantedAngleChange;
use game::snake::StateChange::Death;
use game::snake::StateChange::Spawn;
use game::config::NUMBER_OF_STARTING_PARTS;
use game::config::SEND_ROTATION_INTERVAL;
use game::config::SEND_POSITION_INTERVAL;
use game::config::WORLD_UPDATE_MS;

#[allow(dead_code)]
pub struct Snake {
    pub id: u16,
    pub angle: f32,
    pub wanted_angle: f32,
    pub accelerated: bool,
    pub speed: f32,
    pub send_rotation_time_accumulator: f32,
    pub send_position_time_accumulator: f32,
    pub parts: VecDeque<Point>,
    pub updates: Vec<StateChange>
}

impl Default for Snake {
    fn default() -> Snake {
        Snake {
            id: 0,
            angle: 0.0,
            wanted_angle: 0.0,
            accelerated: false,
            speed: NORMAL_SPEED_PIXELS_PER_SECOND,
            send_rotation_time_accumulator: 0.0,
            send_position_time_accumulator: 0.0,
            parts: VecDeque::new(),
            updates: Vec::new()
        }
    }
}

impl Snake {
    pub fn new_spawned(id: &u16, spawn_point: &Point) -> Snake {
        let mut snake = Snake::default();
        snake.id = *id;
        snake.updates.push(Spawn);
        snake.speed = NORMAL_SPEED_PIXELS_PER_SECOND;
        for i in 0..NUMBER_OF_STARTING_PARTS {
            snake.parts.push_back(Point {
                x: spawn_point.x - i as f32 * snake.speed / 1000.0 * WORLD_UPDATE_MS as f32,
                y: spawn_point.y - i as f32 * snake.speed / 1000.0 * WORLD_UPDATE_MS as f32,
            })
        }
        snake
    }
}

impl Snake {

    pub fn head(&self) -> &Point {
        self.parts.front().unwrap()
    }

    pub fn tail(&self) -> &Point {
        self.parts.back().unwrap()
    }

    pub fn change_angle(&mut self, angle: u8) {
        self.wanted_angle = PI * angle as f32 / HALF_MAX_ANGLE;
        self.updates.push(WantedAngleChange);
    }

    pub fn change_acceleration(&mut self, _is_accelerating: bool) {
        // TODO
    }

    pub fn kill(&mut self) {
        self.updates.push(Death)
    }

    pub fn update(&mut self, delta_time: f32) {
        self.update_rotation(delta_time);
        self.update_movement(delta_time);
    }

    fn update_rotation(&mut self, delta_time: f32) {
        if self.wanted_angle == self.angle {
            return ()
        }
        let rotation_amount = SNAKE_ANGULAR_SPEED_PER_MS * delta_time as f32;
        let angle_difference = (self.wanted_angle - self.angle).normalize_angle().to_plus_minus_pi_range();

        self.angle = match angle_difference {
            angle if angle.abs() < rotation_amount => self.wanted_angle,
            angle if angle > 0.0 => self.angle + rotation_amount,
            _ => self.angle - rotation_amount
        }.normalize_angle();

        self.send_rotation_time_accumulator += delta_time;
        if self.send_rotation_time_accumulator > SEND_ROTATION_INTERVAL as f32 {
            self.updates.push(StateChange::AngleChange);
            self.send_rotation_time_accumulator = 0.0;
        }
    }

    fn update_movement(&mut self, delta_time: f32) {
        self.update_position(delta_time);
        self.update_speed(delta_time);

        self.send_position_time_accumulator += delta_time;
        if self.send_position_time_accumulator >= SEND_POSITION_INTERVAL as f32 {
            self.updates.push(StateChange::PositionChange);
            self.send_position_time_accumulator = 0.0;
        }
    }

    fn update_position(&mut self, delta_time: f32) {
        let travelled_distance: f32 = self.speed / 1000.0 * delta_time;
        let new_head = {
            let head = self.head();
            Point {
                x: head.x + self.angle.cos() * travelled_distance,
                y: head.y + self.angle.sin() * travelled_distance,
            }
        };
        self.parts.pop_back();
        self.parts.push_front(new_head);
    }

    fn update_speed(&mut self, delta_time: f32) {
        let wanted_speed = if self.accelerated {
            ACCELERATED_SPEED_PIXELS_PER_SECOND
        } else {
            NORMAL_SPEED_PIXELS_PER_SECOND
        };

        if self.speed != wanted_speed {
            let speed_difference = wanted_speed - self.speed;
            let acceleration = (ACCELERATION_PER_MILLISECOND as f32 * delta_time) as f32;
            self.speed = match speed_difference {
                difference if difference.abs() < acceleration => wanted_speed,
                difference if difference > 0.0 => self.speed + acceleration,
                _ => self.speed - acceleration
            } as f32;
            self.updates.push(StateChange::SpeedChange);
        }
    }

    pub fn check_collision(&mut self) {
        if !self.head().is_in_world_bounds() {
            self.updates.push(Death);
        }
    }

}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum StateChange {
    AngleChange,
    WantedAngleChange,
    PositionChange,
    SpeedChange,
    Death,
    Spawn
}