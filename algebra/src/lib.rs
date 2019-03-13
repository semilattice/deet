use std::collections::BTreeMap;
use std::collections::BTreeSet;

////////////////////////////////////////////////////////////////////////////////
// Traits

pub trait Semilattice {
    fn join(self, Self) -> Self;
}

pub trait Bounded: Semilattice {
    fn least() -> Self;
}

////////////////////////////////////////////////////////////////////////////////
// BTreeSet

impl<K> Semilattice for BTreeSet<K>
    where K: Ord {
    fn join(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}

impl<K> Bounded for BTreeSet<K>
    where K: Ord {
    fn least() -> Self {
        BTreeSet::new()
    }
}

////////////////////////////////////////////////////////////////////////////////
// BTreeMap

impl<K, V> Semilattice for BTreeMap<K, V>
    where K: Ord, V: Semilattice {
    fn join(mut self, other: Self) -> Self {
        for (k, b) in other {
            if let Some(a) = self.remove(&k) {
                self.insert(k, a.join(b));
            } else {
                self.insert(k, b);
            }
        }
        self
    }
}

impl<K, V> Bounded for BTreeMap<K, V>
    where K: Ord, V: Semilattice {
    fn least() -> Self {
        BTreeMap::new()
    }
}
