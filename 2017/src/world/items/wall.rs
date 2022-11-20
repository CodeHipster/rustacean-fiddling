use std::fmt;
use super::super::geometry::Geometry;
use super::Item;

pub struct Wall{
    pub geo:Geometry,
    pub width: i32,
    pub height: i32,
}

impl Item for Wall {
    fn geometry(&self) -> &Geometry{
        &self.geo
    }
}

impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wall: {} width:{}, height:{})", self.geo, self.width, self.height)
    }
}