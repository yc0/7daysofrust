use std::collections::HashMap;
use std::env::args;
fn main() {
    let mut hm = HashMap::new();
    hm.insert(1, "Apple");
    hm.insert(3, "Cargo");
    hm.insert(5, "Eagle");
    let mut rst = match &hm.get(&3) {
        Some(v) => v,
        _ => "None",
    };

   
    println!("{}", rst );

    rst = &hm.get(&3).unwrap();
    println!("{}", rst);
    rst = &hm.get(&4).unwrap_or(&"None");
    println!("{}", rst);
    // rst = &hm.get(&2).unwrap(); // panic
    

    match "3".parse::<f32>() {
        Ok(v) => println!("Ok - {}", v),
        Err(e)=>println!("Error - {}", e),
    }
    match "r3".parse::<f32>() {
        Ok(v) => println!("Ok - {}", v),
        Err(e)=>println!("Error - {}", e),
    }

    match get_args(3) {
        Ok(v)=>println!("Ok - {}", v),   // > cargo run -- a b c d
                                         // would get 'c'
        Err(e)=>println!("Error - {}", e),
    }
}

fn get_args(n:usize) -> Result<String,String> {
    for (i,a) in args().enumerate() {
        if i==n {
            return Ok(a);
        }
    }
    Err("Not Enough Arguments".to_string())
}
