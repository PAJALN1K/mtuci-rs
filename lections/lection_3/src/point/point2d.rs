use super::Point;

#[derive(Debug, Clone)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

impl Point2d {
    pub fn new(x: i32, y: i32) -> Point2d {
        Point2d { x, y }
    }

    pub fn add(self) -> i32 {
        self.x + self.y
    }
}

impl Point for Point2d {
    fn length(&self) -> f64 {
        ((self.x.pow(2) as f64) + (self.y.pow(2) as f64)).sqrt()
    }
}
