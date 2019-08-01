#[derive(Debug)]
pub struct Bed {
    size:u8,
    count:u8,
}
#[derive(Debug)]
pub enum Room {
    Kitchen(u8),
    Bedroom(Bed),
    Lounge,
    Balcony(u8, String),
}
fn main() {
    use self::Room::*;
    let t = Kitchen(3);
    let nxt =Bedroom(Bed{size:200,count:2});
    println!("Hello from the {:?} !", t);

    match t {
        Kitchen(n) => println!("The room is a kitchen with {}", n),
        x=>println!("{:?}", x),
    }
    match nxt {
        Kitchen(n) => println!("The room is a kitchen with {}", n),
        x=>println!("{:?}", x),
    }

    let single = Kitchen(10);
    if let Kitchen(n) = single {
        println!("Its a kitchen with {} cupboards", n);
    }

    let compound = Balcony(10, "Clean".to_string());
    if let Balcony(v, desc) = compound {
        println!("Its a {} ft. and {} balcony", v, desc);
    }
}
