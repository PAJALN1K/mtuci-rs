use super::Point;

pub struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3d {
    pub fn new(x: i32, y: i32, z: i32) -> Point3d {
        Point3d { x, y, z }
    }

    pub fn add(self) -> i32 {
        self.x + self.y +self.z
    }
}

impl Point for Point3d {
    fn length(&self) -> f64 {
        ((self.x.pow(2) as f64) + (self.y.pow(2) as f64)).sqrt()
    }
}