#[snippet = "binary_indexed_tree"]
#[derive(Debug)]
pub struct BIT {
    bits: Vec<u64>,
}

#[snippet = "binary_indexed_tree"]
impl BIT {
    pub fn new(size: usize) -> Self {
        let mut bit_size = 1;
        while bit_size <= size {
            bit_size *= 2;
        }
        BIT {
            bits: vec![0; bit_size+1],
        }
    }

    // i: 1-indexed but returns 0 if i=0 is given.
    pub fn sum(&mut self, i: usize) -> u64 {
        let mut i = i as i64;
        let mut ret = 0;
        while i > 0 {
            ret += self.bits[i as usize];
            i -= i & -i; // Add a last bit with 1
        }
        ret
    }

    // i: 1-indexed
    pub fn add(&mut self, i: usize, value: u64) {
        assert!(i > 0);
        let n = self.bits.len() as i64;
        let mut i = i as i64;
        while i <= n {
            self.bits[i as usize] += value;
            i += i & -i; // Add a last bit with 1
        }
    }
}

#[test]
fn test() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut bit = BIT::new(a.len());
    for i in 0..a.len() {
        bit.add(i+1, a[i]);
    }
    assert_eq!(bit.sum(1), 1);
    assert_eq!(bit.sum(3), 6);
    assert_eq!(bit.sum(6), 21);
}
