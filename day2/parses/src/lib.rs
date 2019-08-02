//! A library for handling money
use std::str::FromStr;

pub mod parse;
use parse::*;



#[derive(Debug,PartialEq)]
pub enum GBPError {
    ParseError(ParseMoneyError),
    OtherError,
}

impl From<ParseMoneyError> for GBPError {
    fn from(p:ParseMoneyError) -> Self {
        GBPError::ParseError(p)
    }
}


/// Parse your money from a string
/// ```
/// use parses::*;
/// let g = "£31.22".parse(); // parse method actually tests something implement FromStr trait
/// assert_eq!(g, Ok(GBP(3122)));
/// ```

#[derive(Debug,PartialEq)]
pub struct GBP(pub i32);

impl FromStr for GBP {
    // type Err = ParseMoneyError;
    type Err = GBPError;
    fn from_str(s:&str) -> Result<Self, Self::Err> {
        Ok(GBP(parse_sym_money(s, '£', 2)?))
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         let g = "£31.22".parse(); // parse method actually tests something implement FromStr trait
//         assert_eq!(g, Ok(GBP(3122)));
//     }
// }
