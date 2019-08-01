
extern crate rand;
use std::ops::Add;
use rand::Rng;

#[derive(Debug)]
pub struct Point {
    x:i32,
    y:i32,
}
impl Point {
    fn random() -> Self {
        let mut tr = rand::thread_rng();
        Point {
            x : tr.gen(),
            y: tr.gen(),
        }
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs:Point) -> Self::Output {
        Point {
            x: self.x+rhs.x,
            y: self.y+rhs.y,
        }
    }
}
fn main() {
    let a = Point{x:10, y:15};
    let b = Point{x:15, y:20};

    println!("a = {:?}", &a);
    let c = a + b;
    println!("c = {:?}", c);
    
    let d = Point::random();
    println!("d = {:?}", d);
}
