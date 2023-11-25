pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, PartialEq)]
pub struct USD(i32);
#[derive(Debug, PartialEq)]
pub struct GBP(i32);
#[derive(Debug, PartialEq)]
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

pub struct Ex {
    cad: f32,
    gbp: f32,
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

pub trait Exchange<F, T> {
    fn convert(&self, _: F) -> T;
}

impl<E, F, T> Exchange<F, T> for E where E: ToUSDv<F> + FromUSDv<T> {
    fn convert(&self, f: F) -> T {
        self.from_uv(self.to_uv(f))
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
        let ex = Ex { cad: 0.7, gbp: 1.3 };
        let c = ex.from_uv(ex.to_uv(g));
        assert_eq!(c, CAD(371));

        let g = GBP(200);
        let ex = Ex { cad: 0.7, gbp: 1.3 };
        let c = ex.convert(g);
        assert_eq!(c, CAD(371));
    }
}
