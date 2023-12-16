use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct USD(i32);

impl Display for USD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = (self.0 as f32) / 100.0;
        if r < 0.0 {
            return write!(f, "-${:.2}", -r);
        }

        write!(f, "${:.2}", r)
    }
}

impl Clone for USD {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let u = USD(230);
        let b = u.clone();

        assert_eq!(u.to_string(), "$2.30".to_string());

        assert_eq!(u, b)
    }
}
