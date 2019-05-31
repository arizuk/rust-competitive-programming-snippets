#[snippet = "lazy_segment_tree"]
pub mod ds {
    pub struct LazySegmentTree<T, F> {
        n: usize,
        init: T,
        f: F,
        pub data: Vec<T>,
        pub lazy: Vec<Option<T>>,
    }

    impl<T, F> LazySegmentTree<T, F>
    where
        T: Copy,
        F: Fn(T, T) -> T,
    {
        pub fn new(size: usize, init: T, f: F) -> Self {
            let mut n = 1;
            while n < size {
                n *= 2;
            }
            LazySegmentTree {
                n: n,
                init: init,
                f: f,
                data: vec![init; n * 2 - 1],
                lazy: vec![None; n * 2 - 1],
            }
        }

        pub fn range_update(&mut self, l: usize, r: usize, x: T) {
            let n = self.n;
            self.do_range_update(l, r, x, 0, 0, n);
        }

        fn do_range_update(&mut self, l: usize, r: usize, x: T, k: usize, a: usize, b: usize) {
            self.propagate(k);
            if b <= l || r <= a {
                // noop
            } else if l <= a && b <= r {
                self.lazy[k] = Some(x);
                self.propagate(k);
            } else {
                self.do_range_update(l, r, x, k * 2 + 1, a, (a + b) / 2);
                self.do_range_update(l, r, x, k * 2 + 2, (a + b) / 2, b);
                self.data[k] = (self.f)(self.data[k * 2 + 1], self.data[k * 2 + 2]);
            }
        }

        fn propagate(&mut self, k: usize) {
            if let Some(x) = self.lazy[k] {
                if k < self.n - 1 {
                    self.lazy[k * 2 + 1] = Some(x);
                    self.lazy[k * 2 + 2] = Some(x);
                }
                self.data[k] = x;
                self.lazy[k] = None;
            }
        }

        /// [l, r)
        pub fn query(&mut self, l: usize, r: usize) -> T {
            assert!(l < r);
            let n = self.n;
            self.do_query(l, r, 0, 0, n)
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
                (self.f)(q1, q2)
            }
        }
    }
}

#[test]
fn test_segment_tree() {
    use std::cmp::min;
    const MAX: usize = 10000;
    let mut seg_tree = ds::LazySegmentTree::new(8, MAX, |a, b| min(a, b));
    seg_tree.range_update(0, 5, 10);
    seg_tree.range_update(3, 8, 8);
    assert_eq!(seg_tree.query(0, 3), 10);
    assert_eq!(seg_tree.query(0, 8), 8);

    seg_tree.range_update(0, 8, 6);
    assert_eq!(seg_tree.query(7, 8), 6);
}

#[test]
fn test_random() {
    use rand::Rng;
    use std::cmp::min;
    const MAX: usize = 1000;
    const SIZE: usize = 100;
    let mut seg_tree = ds::LazySegmentTree::new(SIZE, MAX, |a, b| min(a, b));
    let mut data = vec![MAX; SIZE];

    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let l = rng.gen_range(0, SIZE);
        let r = rng.gen_range(l, SIZE) + 1;
        let v = rng.gen_range(0, MAX);
        seg_tree.range_update(l, r, v);
        for i in l..r {
            data[i] = v;
        }
        let l = rng.gen_range(0, SIZE);
        let r = rng.gen_range(l, SIZE) + 1;
        let q1 = seg_tree.query(l, r);
        let q2 = **&data[l..r].iter().min().unwrap();
        assert_eq!(q1, q2);
    }
}
