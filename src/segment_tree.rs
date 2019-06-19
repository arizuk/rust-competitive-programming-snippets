#[snippet = "segment_tree"]
pub mod ds {
    pub struct SegmentTree<T, F> {
        n: usize,
        pub data: Vec<T>,
        init: T,
        f: F,
    }

    impl<T, F> SegmentTree<T, F>
    where
        T: Copy,
        F: Fn(T, T) -> T,
    {
        pub fn new(size: usize, init: T, f: F) -> Self {
            let mut n = 1;
            while n < size {
                n *= 2;
            }
            SegmentTree {
                n: n,
                init: init,
                f: f,
                data: vec![init; n * 2 - 1],
            }
        }

        pub fn update(&mut self, mut k: usize, x: T) {
            k += self.n - 1;
            self.data[k] = (self.f)(self.data[k], x);
            while k > 0 {
                k = (k - 1) / 2;
                self.data[k] = (self.f)(self.data[2 * k + 1], self.data[2 * k + 2]);
            }
        }

        pub fn set(&mut self, mut k: usize, x: T) {
            k += self.n - 1;
            self.data[k] = x;
            while k > 0 {
                k = (k - 1) / 2;
                self.data[k] = (self.f)(self.data[2 * k + 1], self.data[2 * k + 2]);
            }
        }

        /// [l, r)
        pub fn query(&self, l: usize, r: usize) -> T {
            assert!(l < r);
            self.do_query(l, r, 0, 0, self.n)
        }

        fn do_query(&self, l: usize, r: usize, k: usize, a: usize, b: usize) -> T {
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
    let mut seg_tree = ds::SegmentTree::new(3, 0usize, |a, b| a + b);
    seg_tree.set(0, 1);
    seg_tree.set(1, 2);
    seg_tree.set(2, 3);

    assert_eq!(seg_tree.query(0, 3), 6);
    assert_eq!(seg_tree.query(2, 3), 3);
}

#[test]
fn test_range_sum_random() {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    const SIZE: usize = 100;
    let mut seg_tree = ds::SegmentTree::new(SIZE, 0usize, |a, b| a + b);
    let mut data = vec![0; SIZE];

    for _ in 0..1000 {
        let i = rng.gen_range(0, SIZE);
        let v = rng.gen_range(0, 50);
        seg_tree.update(i, v);

        data[i] += v;
        let l = rng.gen_range(0, SIZE);
        let r = rng.gen_range(l, SIZE) + 1;

        let q1 = seg_tree.query(l, r);
        let q2 = *&data[l..r].iter().sum();
        assert_eq!(q1, q2);
    }
}

#[test]
fn test_range_minimum_query_random() {
    use rand::Rng;
    use std::cmp::min;

    const MAX: usize = 1e4 as usize;
    const SIZE: usize = 100;

    let mut rng = rand::thread_rng();
    let mut seg_tree = ds::SegmentTree::new(SIZE, MAX, |a, b| min(a, b));
    let mut data = vec![MAX; SIZE];

    for _ in 0..1000 {
        let i = rng.gen_range(0, SIZE);
        let v = rng.gen_range(0, 10000);
        seg_tree.set(i, v);

        data[i] = v;
        let l = rng.gen_range(0, SIZE);
        let r = rng.gen_range(l, SIZE) + 1;

        let q1 = seg_tree.query(l, r);
        let q2 = *&data[l..r].iter().fold(MAX, |a, b| min(a, *b));
        assert_eq!(q1, q2);
    }
}
