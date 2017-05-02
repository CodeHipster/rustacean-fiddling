//import module 'world' in this file.
mod world;

fn main() {
    let world = world::world::World::new();
    //world.add_wall();
    println!("Printing the world:\n{}", world);
}

