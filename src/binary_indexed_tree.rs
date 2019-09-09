#[snippet = "binary_indexed_tree"]
pub mod ds {
    use std::cmp::{min, PartialOrd};
    use std::ops::{AddAssign, Sub};

    #[derive(Debug)]
    pub struct BIT<T> {
        size: usize,
        data: Vec<T>,
    }

    impl<T> BIT<T>
    where
        T: Copy + AddAssign + Sub<Output = T> + PartialOrd + From<usize>,
    {
        pub fn new(size: usize) -> Self {
            let buf_size = size.next_power_of_two();
            BIT {
                size: size,
                data: vec![T::from(0 as usize); buf_size + 1],
            }
        }

        /// [l, r) l,r: 1-indexed
        pub fn sum_between(&self, l: usize, r: usize) -> T {
            self.sum(r - 1) - self.sum(l - 1)
        }

        /// i: 1-indexed but returns 0 if i=0 is given.
        pub fn sum(&self, i: usize) -> T {
            let mut i = i as i64;
            let mut ret = T::from(0 as usize);
            while i > 0 {
                ret += self.data[i as usize];
                i -= i & -i;
            }
            ret
        }

        /// i: 1-indexed
        pub fn add(&mut self, i: usize, value: T) {
            assert!(i > 0 && i <= self.size);
            let n = self.data.len() as i64;
            let mut i = i as i64;
            while i <= n - 1 {
                self.data[i as usize] += value;
                i += i & -i; // Add a last bit with 1
            }
        }

        pub fn lower_bound(&self, mut value: T) -> usize {
            let zero = T::from(0usize);
            if value <= zero {
                return 0;
            }

            let n = self.data.len();
            let mut x = 0;
            let mut k = n - 1;
            while k > 0 {
                if x + k <= n - 1 && self.data[x + k] < value {
                    value = value - self.data[x + k];
                    x += k;
                }
                k /= 2;
            }

            x = min(x, self.size);
            x + 1
        }
    }
}

#[test]
fn test() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut bit = ds::BIT::new(a.len());
    for i in 0..a.len() {
        bit.add(i + 1, a[i]);
    }
    assert_eq!(bit.sum(1), 1);
    assert_eq!(bit.sum(3), 6);
    assert_eq!(bit.sum(6), 21);

    assert_eq!(bit.sum_between(3, 7), 18);
}

#[test]
fn test_size() {
    for size in 1..20 {
        let mut bit = ds::BIT::new(size);
        let mut ans = 0;
        for i in 1..size + 1 {
            bit.add(i, 1);
            ans += 1;
            assert_eq!(bit.sum(i), ans);
        }
    }
}

#[test]
fn test_random() {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    const SIZE: usize = 100;
    let mut bit = ds::BIT::new(SIZE);
    let mut data = vec![0; SIZE];

    for _ in 0..1000 {
        let i = rng.gen_range(0, SIZE);
        let v = rng.gen_range(0, 50);
        bit.add(i + 1, v);
        data[i] += v;

        let l = rng.gen_range(0, SIZE);
        let r = rng.gen_range(l, SIZE) + 1;

        let q1 = bit.sum_between(l + 1, r + 1);
        let q2 = *&data[l..r].iter().sum();
        assert_eq!(q1, q2);
    }
}

#[test]
fn test_lower_bound() {
    {
        let mut bit = ds::BIT::new(8);
        for i in 1..8 + 1 {
            bit.add(i, i);
        }
        assert_eq!(bit.lower_bound(0), 0);
        assert_eq!(bit.lower_bound(1), 1);
        assert_eq!(bit.lower_bound(6), 3);
        assert_eq!(bit.lower_bound(7), 4);
        assert_eq!(bit.lower_bound(8), 4);
        assert_eq!(bit.lower_bound(9), 4);
        assert_eq!(bit.lower_bound(10), 4);
        assert_eq!(bit.lower_bound(15), 5);
        assert_eq!(bit.lower_bound(36), 8);
        assert_eq!(bit.lower_bound(37), 9);
    }

    {
        let mut bit = ds::BIT::new(6);
        for i in 1..6 + 1 {
            bit.add(i, 1);
        }
        assert_eq!(bit.lower_bound(1), 1);
        assert_eq!(bit.lower_bound(6), 6);
        assert_eq!(bit.lower_bound(7), 7);
    }
}
