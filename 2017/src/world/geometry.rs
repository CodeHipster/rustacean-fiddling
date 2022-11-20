use std::fmt;
pub struct Geometry{
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position(x:{}, y:{})", self.x, self.y)
    }
}