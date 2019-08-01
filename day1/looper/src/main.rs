fn main() {
    println!("Hello, world!");
    loopto20();
    array_loop();
}

fn loopto20() {
    for n in 0..10 {
        if n&1 == 0 {
            continue;
        }
        println!("Hello {}", n);
    }
}

fn array_loop() {
    let mut v = Vec::new();
    v.push(4);
    v.push(7);
    v.push(9);
    for n in v {
        println!("{}", n);
    }

    let vv = vec![1,2,3,4];
    for n in vv {
        println!("{}", n);
    }
}