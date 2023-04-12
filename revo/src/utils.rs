#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn distance_euclid(first: &Self, second: &Self) -> i64 {
        let x = second.x as i64 - first.x as i64;
        let y = second.y as i64 - first.y as i64;
        (x * x) + (y * y)
    }

    pub fn distance_manhattan(first: &Self, second: &Self) -> i32 {
        let x = second.x - first.x;
        let y = second.y - first.y;
        x.abs() + y.abs()
    }

    pub fn as_f32(&self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }

    pub fn normalized_distance_between_points(first: &Self, second: &Self) -> (f64, f64) {
        if first.x == second.x && first.y == second.y {
            return (0.0, 0.0);
        }

        let d = (Self::distance_euclid(first, second) as f64).sqrt();

        let dx = (second.x - first.x) as f64 / d;
        let dy = (second.y - first.y) as f64 / d;

        (dx, dy)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_euclid() {
        let first = Coord { x: 0, y: 0 };
        let second = Coord { x: 3, y: 4 };
        assert_eq!(Coord::distance_euclid(&first, &second), 25);
    }

    #[test]
    fn test_distance_manhattan() {
        let first = Coord { x: 0, y: 0 };
        let second = Coord { x: 3, y: 4 };
        assert_eq!(Coord::distance_manhattan(&first, &second), 7);
    }

    #[test]
    fn test_normalized_distance_between_points() {
        // tolerance for floating point comparison
        let eps = 1e-9;

        // Same point
        let first = Coord { x: 1, y: 0 };
        let second = Coord { x: 1, y: 0 };
        assert_eq!(
            Coord::normalized_distance_between_points(&first, &second),
            (0.0, 0.0)
        );

        // -90 degrees
        let first = Coord { x: 2, y: 4 };
        let second = Coord { x: 2, y: 2 };
        assert_eq!(
            Coord::normalized_distance_between_points(&first, &second),
            (0.0, -1.0)
        );

        // +90 degrees
        let first = Coord { x: 2, y: 2 };
        let second = Coord { x: 2, y: 4 };
        assert_eq!(
            Coord::normalized_distance_between_points(&first, &second),
            (0.0, 1.0)
        );

        // -45 degrees (x and y == -1/sqrt(2))
        let first = Coord { x: 5, y: 5 };
        let second = Coord { x: 2, y: 2 };
        let (x, y) = Coord::normalized_distance_between_points(&first, &second);
        assert!((x - -1.0 / (2.0f64).sqrt()).abs() < eps);
        assert!((y - -1.0 / (2.0f64).sqrt()).abs() < eps);

        // +45 degrees (x and y == 1/sqrt(2))
        let first = Coord { x: 2, y: 2 };
        let second = Coord { x: 5, y: 5 };
        let (x, y) = Coord::normalized_distance_between_points(&first, &second);
        assert!((x - 1.0 / (2.0f64).sqrt()).abs() < eps);
        assert!((y - 1.0 / (2.0f64).sqrt()).abs() < eps);
    }
}
