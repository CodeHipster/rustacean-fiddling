use std::fmt;
use super::super::geometry::Geometry;
use super::Item;

pub struct Wall{
    pub g:Geometry,
    pub w: i32,
    pub h: i32,
}

impl Item for Wall {
    fn geometry(&self) -> &Geometry{
        &self.g
    }
}

impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wall: {} w:{}, h:{})", self.g, self.w, self.h)
    }
}