#[snippet = "binary_indexed_tree"]
pub mod bit {
    use std::ops::{AddAssign, Sub};

    #[derive(Debug)]
    pub struct BIT<T> {
        bits: Vec<T>,
        init: T
    }

    impl <T> BIT <T>
    where T: Copy + AddAssign + Sub<Output = T>
    {
        pub fn new(size: usize, init: T) -> Self {
            let bit_size = size.next_power_of_two();
            BIT {
                bits: vec![init; bit_size + 1],
                init: init,
            }
        }

        // [l, r] l,r: 1-indexed
        pub fn range_sum(&self, l: usize, r: usize) -> T {
            self.sum(r) - self.sum(l-1)
        }

        // i: 1-indexed but returns 0 if i=0 is given.
        pub fn sum(&self, i: usize) -> T {
            let mut i = i as i64;
            let mut ret = self.init;
            while i > 0 {
                ret += self.bits[i as usize];
                i -= i & -i;
            }
            ret
        }

        // i: 1-indexed
        pub fn add(&mut self, i: usize, value: T) {
            assert!(i > 0);
            let n = self.bits.len() as i64;
            let mut i = i as i64;
            while i <= n {
                self.bits[i as usize] += value;
                i += i & -i; // Add a last bit with 1
            }
        }
    }
}

#[test]
fn test() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut bit = bit::BIT::new(a.len(), 0);
    for i in 0..a.len() {
        bit.add(i + 1, a[i]);
    }
    assert_eq!(bit.sum(1), 1);
    assert_eq!(bit.sum(3), 6);
    assert_eq!(bit.sum(6), 21);

    assert_eq!(bit.range_sum(3, 6), 18);
}
