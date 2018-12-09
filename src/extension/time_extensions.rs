use std::time::SystemTime;

pub trait TimeExtension {

    fn millis_since(&self, other: SystemTime) -> f32;

}

impl TimeExtension for SystemTime {

    fn millis_since(&self, other: SystemTime) -> f32 {
        let duration_since = self.duration_since(other).unwrap();
        return (duration_since.as_secs() * 1000) as f32 + duration_since.subsec_nanos() as f32 / 1_000_000.0;
    }

}