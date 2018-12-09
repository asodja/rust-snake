
use std::f32::consts::PI;


pub trait NumberExtension {

    fn normalize_angle(&self) -> f32;

    fn to_plus_minus_pi_range(&self) -> f32;

    fn is_clockwise(&self) -> bool;

}

impl NumberExtension for f32 {

    fn normalize_angle(&self) -> f32 {
        self - 2.0 * PI as f32 * (self / (2.0 * PI)).floor()
    }

    fn to_plus_minus_pi_range(&self) -> f32 {
        if *self > PI as f32 { self - 2.0 * PI } else { *self }
    }

    fn is_clockwise(&self) -> bool {
        self.normalize_angle().to_plus_minus_pi_range() > 0.0
    }

}