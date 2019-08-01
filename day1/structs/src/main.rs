#[derive(Debug)]
struct User {
    name:String,
    age:u8,
    height:u8,
    shoe_size:u8,
}
impl User {
    fn to_str(&self) -> String {
        format!("{} - {} - {} cm - shoe:{}", self.name,
            self.age,
            self.height,
            self.shoe_size)
    }
    fn grow(&mut self, h : u8) {
        self.height += h;
    }
    fn die(self) {
        println!("{} die at {}", self.name, self.age)
    }
}
fn main() {
    let mut user = User {
        name:"Matt".to_string(),
        age:40,
        height:190,
        shoe_size:11,
    };
    println!("Hello, world!");
    println!("User is {:?}", user );
    println!("User is {}", user.to_str() );
    user.grow(10);
    println!("User is {}", user.to_str() );
    user.die(); 
    // println!("User is {}", user.to_str() ); //value borrowed here after move
    
}
