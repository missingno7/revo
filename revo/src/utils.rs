#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn distance(first: &Self, second: &Self) -> f64 {
        let x = second.x as f64 - first.x as f64;
        let y = second.y as f64 - first.y as f64;
        (x * x) + (y * y)
    }

    pub fn as_f32(&self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}
