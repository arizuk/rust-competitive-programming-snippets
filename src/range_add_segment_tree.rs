#[snippet = "range_add_segment_tree"]
pub mod ds {
    use std::cmp::PartialOrd;
    use std::convert::From;
    use std::ops::{Add, AddAssign, Div, Mul};
    pub struct RangeAddSegmentTree<T> {
        n: usize,
        init: T,
        pub data: Vec<T>,
        pub lazy: Vec<T>,
    }

    impl<T> RangeAddSegmentTree<T>
    where
        T: Copy
            + Add<Output = T>
            + AddAssign
            + Div<Output = T>
            + Mul<Output = T>
            + From<u64>
            + PartialOrd,
    {
        pub fn new(size: usize, init: T) -> Self {
            let mut n = 1;
            while n < size {
                n *= 2;
            }
            RangeAddSegmentTree {
                n: n,
                init: init,
                data: vec![init; n * 2 - 1],
                lazy: vec![init; n * 2 - 1],
            }
        }

        pub fn range_add(&mut self, l: usize, r: usize, x: T) {
            self.do_range_add(l, r, x, 0, 0, self.n);
        }

        fn do_range_add(&mut self, l: usize, r: usize, x: T, k: usize, a: usize, b: usize) {
            self.propagate(k);
            if b <= l || r <= a {
                // noop
            } else if l <= a && b <= r {
                self.lazy[k] += x * T::from((b - a) as u64);
                self.propagate(k);
            } else {
                self.do_range_add(l, r, x, k * 2 + 1, a, (a + b) / 2);
                self.do_range_add(l, r, x, k * 2 + 2, (a + b) / 2, b);
                self.data[k] = self.data[k * 2 + 1] + self.data[k * 2 + 2];
            }
        }

        fn propagate(&mut self, k: usize) {
            if self.lazy[k] > self.init {
                let two = T::from(2u64);
                if k < self.n - 1 {
                    let value = self.lazy[k] / two;
                    self.lazy[k * 2 + 1] += value;
                    self.lazy[k * 2 + 2] += value;
                }
                self.data[k] += self.lazy[k];
                self.lazy[k] = self.init;
            }
        }

        /// [l, r)
        pub fn query(&mut self, l: usize, r: usize) -> T {
            assert!(l < r);
            self.do_query(l, r, 0, 0, self.n)
        }

        fn do_query(&mut self, l: usize, r: usize, k: usize, a: usize, b: usize) -> T {
            self.propagate(k);
            if b <= l || r <= a {
                self.init
            } else if l <= a && b <= r {
                self.data[k]
            } else {
                let q1 = self.do_query(l, r, k * 2 + 1, a, (a + b) / 2);
                let q2 = self.do_query(l, r, k * 2 + 2, (a + b) / 2, b);
                q1 + q2
            }
        }
    }
}

#[test]
fn test_segment_tree() {
    let mut seg_tree = ds::RangeAddSegmentTree::new(8, 0u64);
    seg_tree.range_add(0, 4, 5);
    assert_eq!(seg_tree.query(0, 4), 20);
    assert_eq!(seg_tree.query(2, 4), 10);
    assert_eq!(seg_tree.query(3, 4), 5);

    seg_tree.range_add(3, 5, 1);
    assert_eq!(seg_tree.query(0, 4), 21);
    assert_eq!(seg_tree.query(2, 8), 12);
}

#[test]
fn test_random() {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    const SIZE: usize = 100;
    let mut seg_tree = ds::RangeAddSegmentTree::new(SIZE, 0u64);
    let mut data = vec![0; SIZE];

    for _ in 0..1000 {
        let l = rng.gen_range(0, SIZE);
        let r = rng.gen_range(l, SIZE) + 1;
        let v = rng.gen_range(0, 50);
        seg_tree.range_add(l, r, v);
        for i in l..r {
            data[i] += v;
        }

        let l = rng.gen_range(0, SIZE);
        let r = rng.gen_range(l, SIZE) + 1;
        let q1 = seg_tree.query(l, r);
        let q2 = *&data[l..r].iter().sum();
        assert_eq!(q1, q2);
    }
}
