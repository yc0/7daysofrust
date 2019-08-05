pub fn trim_left<'a>(s:&'a str) -> &'a str {
    for (i,c) in s.char_indices() {
        if c == ' ' {
            continue;
        }
        println!("{}", s.chars().nth(i).unwrap());
        return s.get(i..s.len()).unwrap();
    }
    ""
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(trim_left("  台灣, Hello"),"台灣, Hello");
        let mut s = "  Hello".to_string();
        {
            let s2 = trim_left(&s);
            assert_eq!(s2,"Hello");
        }
        s.push_str(" World");
        assert_eq!(trim_left(&s), "Hello World");
    }
}
