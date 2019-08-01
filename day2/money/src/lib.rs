#[derive(PartialEq, Debug, Clone)]
pub struct USD (i32);
#[derive(PartialEq, Debug, Clone)]
pub struct CAD (i32);
#[derive(PartialEq, Debug, Clone)]
pub struct GBP (i32);

pub trait ToUSD {
    fn to_usd(&self) -> USD;
    fn convert<T:FromUSD>(&self)->T {
        T::from_usd(&self.to_usd())
    }
}
pub trait FromUSD {
    fn from_usd(_:&USD)->Self;
}


pub trait ToUSDv<F> {
    fn to_uv(&self, _:F) -> f32;
}

pub trait FromUSDv<F> {
    fn from_uv(&self, _:f32) -> F;
}
#[derive(PartialEq,Debug)]
pub struct Transaction<A> {
    from_id : i32,
    to_id : i32,
    amount : A,
}

pub trait Account {
    fn id(&self) -> i32;
}
pub struct Ex {
    ac_id : i32,
    cad : f32,   // conversion rate
    gbp : f32,
}

impl ToUSD for GBP {
    fn to_usd(&self) -> USD {
        USD(self.0 * 130 / 100)
    }
}
impl FromUSD for CAD {
    fn from_usd(u:&USD) -> Self {
        CAD((u.0 * 130) / 100)
    }
}

impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g:GBP) -> f32 {
        (g.0 as f32) * self.gbp
    }
}

impl FromUSDv<CAD> for Ex {
    fn from_uv(&self, f:f32) -> CAD{
        CAD( (f/self.cad) as i32)
    }
}

impl Account for Ex {
    fn id(&self) -> i32 {
        self.ac_id
    }
}
pub trait Exchange<F,T> {
    fn convert(&self, _:F) -> T;
}

impl<E,F,T> Exchange<F,T> for E
    where E:ToUSDv<F>+FromUSDv<T> {
    fn convert(&self, f:F) -> T {
        self.from_uv(self.to_uv(f))
    }
}


pub trait ExchangeAccount<F,T> {
    fn exchange(&self, _:i32, _:i32, _:F) -> (Transaction<F>,Transaction<T>);
}

impl<E,F,T> ExchangeAccount<F,T> for E 
    where E:Exchange<F,T> + Account,
          F:Clone {

    fn exchange(&self, f_id:i32, t_id:i32, amount:F) -> (Transaction<F>,Transaction<T>) {
        let ft = Transaction{from_id : f_id, to_id:self.id(), amount: amount.clone()};// from transaction
        let tt = Transaction{from_id : self.id(), to_id: t_id, amount:self.convert(amount)};// to transaction
        (ft,tt)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let g = GBP(200);
        let u = g.to_usd();
        
        assert_eq!(u, USD(260));
        let c = CAD::from_usd(&u);
        assert_eq!(c, CAD(338) );
        // assert_eq!(2 + 2, 4);

        let c2:CAD = g.convert();
        assert_eq!(c , c2);
    }

    #[test]
    fn generic_parameters() {
        let g = GBP(200);
        let ex = Ex{ac_id:0, cad:0.7, gbp:1.3};
        let c = ex.from_uv(ex.to_uv(g));
        assert_eq!(c, CAD(371));

        let g2 = GBP(200);
        let c2:CAD = ex.convert(g2);
        assert_eq!(c2, CAD(371));
    }

    #[test]
    fn generic_structs() {
        let ex = Ex{ac_id:30, cad:0.7, gbp:1.3};
        let (ft,tt) = ex.exchange(20,40,GBP(200));
        assert_eq!(ft, Transaction{from_id:20, to_id:30, amount:GBP(200)});
        assert_eq!(tt, Transaction{from_id:30, to_id:40, amount:CAD(371)});
    }
}
