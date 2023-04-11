#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn distance_euclid(first: &Self, second: &Self) -> i64 {
        let x = second.x as i64 - first.x as i64;
        let y = second.y as i64 - first.y as i64;
        (x * x) + (y * y)
    }

    pub fn distance_manhattan(first: &Self, second: &Self) -> i32 {
        let x = second.x as i32 - first.x as i32;
        let y = second.y as i32 - first.y as i32;
        x.abs() + y.abs()
    }

    pub fn as_f32(&self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}

#[derive(Debug, PartialEq)]
pub struct LabData {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

pub struct IndexedLabData {
    pub index: usize,
    pub data: LabData,
}

impl IndexedLabData {
    pub fn new(l: f64, a: f64, b: f64, index: usize) -> Self {
        IndexedLabData {
            data: LabData { l, a, b },
            index,
        }
    }
}
