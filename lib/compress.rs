mod compress {
    use std::{collections::{BTreeSet, BinaryHeap}, fmt::Binary};

    use amplify::default;
    use itertools::Itertools;
    use permutohedron::Heap;

    pub struct Compress<T> {
        restore: Vec<T>,
    }

    impl<T: Copy + Ord> Compress<T> {
        pub fn new(set: Vec<T>) -> Self {
            Self { restore: set }
        }

        pub fn size(&self) -> usize {
            self.restore.len()
        }

        pub fn compress(&self, x: T) -> usize {
            self.restore.binary_search(&x).unwrap()
        }

        pub fn restore(&self, x: usize) -> T {
            self.restore[x]
        }
    }
}
use compress::*;