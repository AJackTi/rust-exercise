use std::ops::AddAssign;
use std::cmp::PartialOrd;

pub struct Stepper<T> {
    curr: T,
    stop: T,
    step: T,
}

impl<T> Stepper<T> {
    pub fn new(curr: T, stop: T, step: T) -> Stepper<T> {
        Stepper {
            curr,
            stop,
            step,
        }
    }
}

impl<T> Iterator for Stepper<T> where T: AddAssign + PartialOrd + Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.stop {
            return None;
        }

        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn sum_list<I, S>(l: I, mut s: S) -> S where I: Iterator<Item = S>, S: AddAssign {
    // for n in l {
    //     s += n;
    // }
    // s

    let mut it = l.into_iter();
    while let Some(n) = it.next() {
        s += n;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut c = 0;
        for n in Stepper::new(2, 10, 2) {
            c += n;
        }

        assert_eq!(c, 20);

        let sl = sum_list(Stepper::new(3, 10, 2), 0);
        assert_eq!(sl, 24);

        let fl = Stepper::new(4, 10, 2).fold(0, |a, b| a + b);
        assert_eq!(fl, 18);
    }
}
