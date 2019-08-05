#[derive(Debug, PartialEq)]
pub struct GBP(pub i32);

// fn money_pointer<'a>(i:i32)->&'a GBP {
//     let g = GBP(i);
//     &g
// }

pub fn on_money(a:i32, b:i32)->GBP {
    let g = GBP(a);
    let lhs = &g;
    let rst = GBP(lhs.0+b);
    rst
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // assert_eq!(*mo/ney_pointer(30),GBP(30));
        assert_eq!(on_money(3,4), GBP(7) )
    }
}
