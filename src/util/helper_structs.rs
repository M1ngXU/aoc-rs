use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Copy, Clone, Debug)]
pub struct EqOrdPayload<E: PartialEq + Eq, T>(E, T);

impl<E: PartialEq + Eq, T> PartialEq for EqOrdPayload<E, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<E: PartialEq + Eq, T> Eq for EqOrdPayload<E, T> {}
impl<E: Hash + Eq, T> Hash for EqOrdPayload<E, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<E: PartialOrd + PartialEq + Eq, T> PartialOrd for EqOrdPayload<E, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<E: PartialOrd + Ord + PartialEq + Eq, T> Ord for EqOrdPayload<E, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
