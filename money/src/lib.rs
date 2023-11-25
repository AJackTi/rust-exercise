pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, PartialEq, Clone)]
pub struct USD(i32);
#[derive(Debug, PartialEq, Clone)]
pub struct GBP(i32);
#[derive(Debug, PartialEq, Clone)]
pub struct CAD(i32);

pub trait ToUSD {
    fn to_usd(&self) -> USD;
    fn convert<T: FromUSD>(&self) -> T {
        T::from_usd(&self.to_usd())
    }
}

pub trait FromUSD {
    fn from_usd(u: &USD) -> Self;
}

impl ToUSD for GBP {
    fn to_usd(&self) -> USD {
        USD((self.0 * 130) / 100)
    }
}

impl FromUSD for CAD {
    fn from_usd(u: &USD) -> Self {
        CAD((u.0 * 130) / 100)
    }
}

pub trait ToUSDv<F> {
    fn to_uv(&self, _: F) -> f32;
}

pub trait FromUSDv<F> {
    fn from_uv(&self, _: f32) -> F;
}

impl Account for Ex {
    fn id(&self) -> i32 {
        self.ac_id
    }
}

pub struct Ex {
    ac_id: i32,
    cad: f32,
    gbp: f32,
}

#[derive(Debug, PartialEq)]
pub struct Transaction<A> {
    from_id: i32,
    to_id: i32,
    amount: A,
}

impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g: GBP) -> f32 {
        (g.0 as f32) * self.gbp
    }
}

impl FromUSDv<CAD> for Ex {
    fn from_uv(&self, f: f32) -> CAD {
        CAD((f / self.cad) as i32)
    }
}

pub trait Account {
    fn id(&self) -> i32;
}

pub trait Exchange<F, T> {
    fn convert(&self, _: F) -> T;
}

impl<E, F, T> Exchange<F, T> for E where E: ToUSDv<F> + FromUSDv<T> {
    fn convert(&self, f: F) -> T {
        self.from_uv(self.to_uv(f))
    }
}

pub trait ExchangeAccount<F, T> {
    fn exchange(&self, f_id: i32, i_id: i32, amount: F) -> (Transaction<F>, Transaction<T>);
}

impl<E, F, T> ExchangeAccount<F, T> for E where E: Exchange<F, T> + Account, F: Clone {
    fn exchange(&self, f_id: i32, t_id: i32, amount: F) -> (Transaction<F>, Transaction<T>) {
        let ft = Transaction { from_id: f_id, to_id: self.id(), amount: amount.clone() };
        let tt = Transaction { from_id: self.id(), to_id: t_id, amount: self.convert(amount) };
        (ft, tt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let g = GBP(200);
        let u = g.to_usd();
        assert_eq!(u, USD(260));

        let c = CAD::from_usd(&u);
        assert_eq!(c, CAD(338));

        let c2: CAD = g.convert();
        assert_eq!(c2, c);

        let g = GBP(200);
        let ex = Ex { cad: 0.7, gbp: 1.3, ac_id: 0 };
        let c = ex.from_uv(ex.to_uv(g));
        assert_eq!(c, CAD(371));

        let g = GBP(200);
        let ex = Ex { cad: 0.7, gbp: 1.3, ac_id: 0 };
        let c = ex.convert(g);
        assert_eq!(c, CAD(371));

        let ex = Ex { ac_id: 30, cad: 0.7, gbp: 1.3 };
        let (ft, tt) = ex.exchange(20, 40, GBP(200));
        assert_eq!(ft, Transaction { from_id: 20, to_id: 30, amount: GBP(200) });
        assert_eq!(tt, Transaction { from_id: 30, to_id: 40, amount: CAD(371) });
    }
}
