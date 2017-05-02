use std::fmt;
use super::items::Item;
use super::items::wall::Wall;
use super::geometry::Geometry;

pub struct World{
    //Using a box with a 'trait object' called Item
    //We box Item because we can never know the size of an Item
    //which means that it can only be borrowed. And borrowed means that someone somewhere must be the owner.
    //By boxing the trait object the Box becomes the owner. If the vector ever drops the box, the box and the item it contains will be destroyed.
    pub items: Vec<Box<Item>>
    //should be possible to define the lifetime of the item equal to the vector right?
}

impl World{
    pub fn new () -> World{
        World{items:vec![]}
    }

    //Set self mutable, else we will not be able to mutate the vector.
    pub fn add_wall(&mut self) {
        let wall = Wall{g:Geometry{x:1,y:1}, w:10, h:10};
        //Box the wall, else it would go out of scope when function returns ( at } ).
        &self.items.push(Box::new(wall));
    }
}    
impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        for item in &self.items{
            write!(f, "Items:\n{}", item).unwrap();
        }

        Ok(())
    }
}