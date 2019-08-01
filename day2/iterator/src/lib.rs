use std::ops::AddAssign;
use std::cmp::PartialOrd;

pub struct Stepper<T> {
    cur : T,
    step : T,
    stop : T,
}

impl<T> Stepper<T> {
    pub fn new(start:T, stop:T, step:T) -> Self {
        Stepper {
            cur : start,
            step : step,
            stop : stop,
        }
    }
}

impl<T> Iterator for Stepper<T> 
    where T:AddAssign+Copy+PartialOrd
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.cur >= self.stop {
            return None;
        }
        let rst = self.cur;
        self.cur += self.step;
        Some(rst)
    }
}

fn sum_list<I,S>(l:I, mut s:S) -> S
    where I: Iterator<Item=S>,
          S: AddAssign,
{
    let mut it = l.into_iter();
    while let Some(n) = it.next() {
        s += n;
    } 
    // for n in l {
    //     s += n;
    // }
    s
}

fn sum_list_into<I,S>(l:I, mut s:S) -> S
    where I: IntoIterator<Item=S>,
          S: AddAssign,
{
    let mut it = l.into_iter();
    while let Some(n) = it.next() {
        s += n;
    } 
    // for n in l {
    //     s += n;
    // }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut c = 0;
        for n in Stepper::new(2,10,2) {
            println!("{}", n);
            c += n;
        }
        assert_eq!(c, 20);
    }

    #[test]
    fn it2_works() {
        let sl = sum_list(Stepper::new(3,10,2), 0);
        assert_eq!(sl, 24);

        let sl2 = sum_list_into(Stepper::new(3,10,2), 0);
        assert_eq!(sl2, 24);


        let fl = Stepper::new(4,10,2).fold(0, |acc,x| acc + x);
        // assert_eq!(fl, 19);
        assert_eq!(fl, 18);
    }
}
