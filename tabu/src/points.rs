#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Points {
    pub list: Vec<Point>
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn dist(&self, other: &Self) -> u32 {
        let dist_squared = (self.x as f64 - other.x as f64).powi(2) + (self.y as f64 - other.y as f64).powi(2);

        dist_squared.sqrt().round() as u32
    }
}

impl Points {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn from_points(points: Vec<Point>) -> Self {
        Self { list: points }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: u32, y: u32) -> Point {
        Point::new(x, y)
    }

    #[test]
    fn dist_test() {
        dbg!(p(0,0).dist(&p(2,2)));
    }
}