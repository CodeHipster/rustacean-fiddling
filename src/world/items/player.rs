use std::fmt;
use super::super::geometry::Geometry;
use super::Item;

pub struct Player{
    pub g:Geometry,
    pub radius: i32,
    pub name: String,
}

impl Item for Player {
    fn geometry(&self) -> &Geometry{
        &self.g
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player: {} radius:{}, name:{})", self.g, self.radius, self.name)
    }
}