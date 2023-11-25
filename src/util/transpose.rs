use itertools::Itertools;

pub trait Transpose {
    type Transposed;

    fn t(self) -> Self::Transposed;
}
impl<T> Transpose for Vec<Vec<T>> {
    type Transposed = Self;

    fn t(self) -> Self::Transposed {
        if self.is_empty() {
            return self;
        }
        let mut r = Self::with_capacity(self[0].len());
        for _ in 0..self[0].len() {
            r.push(Vec::with_capacity(self.len()));
        }
        for i in 0..self.len() {
            assert_eq!(self[0].len(), self[i].len());
        }
        let m = self.len();
        let n = self[0].len();
        let mut v = self.into_iter().map(|r| r.into_iter()).collect_vec();
        for i in r.iter_mut().take(n) {
            for j in v.iter_mut().take(m) {
                i.push(j.next().unwrap());
            }
        }
        r
    }
}
