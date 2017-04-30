
//Or pose (translation and orientation in some space.)
pub mod geometry{
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
}

pub mod item{
    use std::fmt;
    use geometry::Geometry;
    pub trait Item : fmt::Display{
        fn geometry(&self) -> &Geometry;
    }
}

pub mod wall{
    use std::fmt;
    use geometry::Geometry;
    use item::Item;

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
}

use wall::Wall;
use geometry::Geometry;
use item::Item;

fn main() {
    let wall0 = create_wall();
    let wall0_ref = &wall0;
    let items = vec![wall0_ref as &Item];
    println!("This is item nr 0 {}", items[0]);
}

fn create_wall() -> Wall {
    Wall{g:Geometry{x:1,y:1}, w:10, h:10}
}