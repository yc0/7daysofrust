use std::env::args;

fn main() {
    for a in args() {
        if a.chars().nth(0).unwrap_or('\0') == 'w' {
             println!("{}: Hello everyone", a);
        }
    }

    for a in args() {
        if let Some(c) = a.chars().next(){
            match c {
                'w' | 'W' => println!("Hello {}", a),
                _=>{}
            }
        }
    }
}
