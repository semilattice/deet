#[macro_export]
macro_rules! crdt_compound {
    ($type:ty, $($field:ident),*) => {
        impl crate::algebra::Semilattice for $type {
            fn join(self, other: Self) -> Self {
                Self{
                    $($field: crate::algebra::Semilattice::join(self.$field, other.$field),)*
                }
            }
        }

        impl crate::algebra::Bounded for $type {
            fn least() -> Self {
                Self{ $($field: crate::algebra::Bounded::least()),* }
            }
        }
    };
}

#[macro_export]
macro_rules! crdt_ord_max {
    ($type:ty) => {
        impl crate::algebra::Semilattice for $type {
            fn join(self, other: Self) -> Self {
                crate::std::cmp::Ord::max(self, other)
            }
        }
    }
}
