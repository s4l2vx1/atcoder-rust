mod binary_search {
    use std::ops::{Bound, RangeBounds};

    pub trait BinarySearch {
        fn partition_point<F: Fn(usize) -> bool>(self, f: F) -> usize;
    }

    impl<T: std::ops::RangeBounds<usize>> BinarySearch for T {
        fn partition_point<F: Fn(usize) -> bool>(self, f: F) -> usize {
            let mut l = match self.start_bound() {
                Bound::Included(&l) => l,
                Bound::Excluded(&l) => l + 1,
                _ => panic!()
            };
            let i = f(l);
            let mut r = match self.end_bound() {
                Bound::Included(&r) => r + 1,
                Bound::Excluded(&r) => r,
                _ => {
                    let mut r = 8192;
                    while f(r) == i { (l, r) = (r, r * 8 + 7); }
                    r
                }
            };
            while r - l > 1 {
                let m = l + (r - l) / 2;
                if f(m) == i {
                    l = m;
                } else {
                    r = m;
                }
            }
            r
        }
    }
}
use binary_search::*;