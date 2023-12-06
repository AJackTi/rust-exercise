pub struct SkipIterator<I: Iterator> {
    inner: I,
}

impl<I, T> Iterator for SkipIterator<I> where I: Iterator<Item = T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()?;
        self.inner.next()
    }
}

pub trait IterCombi: Iterator + Sized {
    fn skip_half(self) -> SkipIterator<Self> {
        SkipIterator { inner: self }
    }
}

impl<I: Iterator + Sized> IterCombi for I {}

#[cfg(test)]
mod test_skip {
    use super::*;
    #[test]
    fn test_skip_half() {
        let v: i32 = (0..10).skip_half().sum();
        assert_eq!(v, 1 + 3 + 5 + 7 + 9);
    }
}
