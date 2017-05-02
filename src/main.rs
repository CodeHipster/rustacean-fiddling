//import module 'world' in this file.
mod world;

fn main() {
    //add_wall requires a mutable world as it will modify the vector containing the items.
    let mut world = world::world::World::new();
    world.add_wall();
    println!("Printing the world:\n{}", world);
}

