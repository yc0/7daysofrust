pub enum Tree<T> {
    Empty,
    Head(T, Box<Tree<T>>, Box<Tree<T>>),
}
use self::Tree::*;
// impl<T> Tree<T> {
    //where T:PartialOrd
impl<T:PartialOrd> Tree<T> {
    pub fn empty() -> Self {
        Empty
    }
    pub fn new(t:T) -> Self {
        Head(t, Box::new(Empty), Box::new(Empty))
    }
    pub fn add(&mut self, t : T) {
        match self {
            Empty => {
                *self = Tree::new(t);
            },
            Head(d, lt, rt) => {
                if t < *d {
                    lt.add(t);
                } else {
                    rt.add(t);
                }
            },
        }
    }
}

impl<T:Clone> Tree<T> {
    pub fn lt_d_rt(&self) -> Vec<T> {
        match self {
            Empty => vec![],
            Head(d, lt, rt) => {
                let mut rst = lt.lt_d_rt();
                rst.push(d.clone());
                rst.append(&mut rt.lt_d_rt());
                rst
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut t = Tree::new(5);
        t.add(4);
        t.add(9);
        t.add(3);
        assert_eq!(t.lt_d_rt(), vec![3,4,5,9]);
    }
}
