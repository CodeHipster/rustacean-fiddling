use std::fmt;
use super::items::Item;
use super::items::wall::Wall;
use super::items::player::Player;
use super::geometry::Geometry;

pub struct World{
    //Using a box with a 'trait object' called Item
    //We box Item because we can never know the size of an Item
    //which means that it can only be borrowed. And borrowed means that someone somewhere must be the owner.
    //By boxing the trait object the Box becomes the owner. If the vector ever drops the box, the box and the item it contains will be destroyed.
    pub items: Vec<Box<Item>>
}

impl World{
    pub fn new () -> World{
        World{items:vec![]}
    }

    //Set self mutable, else we will not be able to mutate the vector.
    pub fn add_wall(&mut self) {
        let wall = Wall{geo:Geometry{x:1,y:1}, width:10, height:10};
        //Box the wall, else it would go out of scope when function returns ( at } ).
        &self.items.push(Box::new(wall));
    }

    //Set self mutable, else we will not be able to mutate the vector.
    pub fn add_player(&mut self) {
        let player = Player{g:Geometry{x:1,y:1}, radius:1, name:String::from("mr awesome.")};
        //Box the wall, else it would go out of scope when function returns ( at } ).
        &self.items.push(Box::new(player));
    }
}    
impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nItems:").unwrap();
        for item in &self.items{
            write!(f, "\n{}", item).unwrap();
        }

        Ok(())
    }
}