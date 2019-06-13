#[snippet = "binary_indexed_tree"]
pub mod ds {
    use std::ops::{AddAssign, Sub};

    #[derive(Debug)]
    pub struct BIT<T> {
        data: Vec<T>,
    }

    impl<T> BIT<T>
    where
        T: Copy + AddAssign + Sub<Output = T> + Default,
    {
        pub fn new(size: usize) -> Self {
            let buf_size = size.next_power_of_two();
            BIT {
                data: vec![T::default(); buf_size + 1],
            }
        }

        /// [l, r) l,r: 1-indexed
        pub fn range_sum(&self, l: usize, r: usize) -> T {
            self.sum(r - 1) - self.sum(l - 1)
        }

        /// i: 1-indexed but returns 0 if i=0 is given.
        pub fn sum(&self, i: usize) -> T {
            let mut i = i as i64;
            let mut ret = T::default();
            while i > 0 {
                ret += self.data[i as usize];
                i -= i & -i;
            }
            ret
        }

        /// i: 1-indexed
        pub fn add(&mut self, i: usize, value: T) {
            assert!(i > 0);
            let n = self.data.len() as i64;
            let mut i = i as i64;
            while i <= n - 1 {
                self.data[i as usize] += value;
                i += i & -i; // Add a last bit with 1
            }
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

    assert_eq!(bit.range_sum(3, 7), 18);
}

#[test]
fn test_size() {
    for size in 1..20 {
        let mut bit = ds::BIT::new(size);
        let mut ans = 0;
        for i in 1..size+1 {
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
        bit.add(i+1, v);
        data[i] += v;

        let l = rng.gen_range(0, SIZE);
        let r = rng.gen_range(l, SIZE) + 1;

        let q1 = bit.range_sum(l+1, r+1);
        let q2 = *&data[l..r].iter().sum();
        assert_eq!(q1, q2);
    }
}
