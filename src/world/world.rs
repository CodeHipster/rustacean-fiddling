use std::fmt;
use super::items::Item;
use super::items::wall::Wall;
use super::geometry::Geometry;

pub struct World<'a>{
    //Using a box with a 'trait object' called Item
    //We box Item because we can never know the size of an Item
    //which means that it can only be borrowed. And borrowed means that someone somewhere must be the owner.
    //By boxing the trait object the Box becomes the owner. If the vector ever drops the box, the box and the item it contains will be destroyed.
    pub items: Vec<&'a Item>
    //should be possible to define the lifetime of the item equal to the vector right?
}

impl<'a> World<'a>{
    pub fn new () -> World<'a>{
        World{items:vec![]}
    }

    pub fn add_wall(&mut self) {
        let wall = Wall{g:Geometry{x:1,y:1}, w:10, h:10};
        &self.items.push(&wall);
    }
}    
impl<'a> fmt::Display for World<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        for item in &self.items{
            write!(f, "Items:{}", item).unwrap();
        }

        Ok(())
    }
}