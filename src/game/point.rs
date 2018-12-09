use game::config::WORLD_WIDTH;
use game::config::WORLD_HEIGHT;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Default for Point {
    fn default() -> Point {
        Point {
            x: 0.0,
            y: 0.0
        }
    }
}

impl Point {

    pub fn is_in_world_bounds(&self) -> bool {
        self.x >= 0.0 && self.x <= WORLD_WIDTH as f32 && self.y >= 0.0 && self.y <= WORLD_HEIGHT as f32
    }

}