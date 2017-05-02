//export modules
// re-export modules so they are visible to modules using this one.
pub mod wall;

use std::fmt;
use super::geometry::Geometry;

pub trait Item : fmt::Display{
    fn geometry(&self) -> &Geometry;
}