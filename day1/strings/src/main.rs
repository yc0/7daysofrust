fn main() {
    let s = String::from("Hello 台灣");
    println!("s Len = {}", s.len());
     println!("#l of s, Len = {}", count_l(&s));
    for c in s.chars() {
        println!("{}", c);
    }
    for (i,c) in s.chars().enumerate() {
        println!("{} = {}", i, c);
    }
    for (i,c) in s.char_indices() {
        println!("{} = {}", i, c);
    }
}

fn count_l(s:&str) -> i32{
    let mut rst = 0;
    for c in s.chars() {
        if c == 'l' {
            rst += 1;
        }
    }
    rst
}
