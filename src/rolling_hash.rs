#[snippet = "rolling_hash"]
pub struct RollingHash2 {
    rh1: RollingHash,
    rh2: RollingHash,
}

#[snippet = "rolling_hash"]
impl RollingHash2 {
    pub fn new(s: &[u8]) -> Self {
        let rh1 = RollingHash::new(s, 1007, 1000000009);
        let rh2 = RollingHash::new(s, 1009, 1000000007);
        RollingHash2 { rh1: rh1, rh2: rh2 }
    }

    pub fn get(&self, l: usize, r: usize) -> (u64, u64) {
        (self.rh1.get(l, r), self.rh2.get(l, r))
    }
}

#[snippet = "rolling_hash"]
pub struct RollingHash {
    modulo: u64,
    power: Vec<u64>,
    hash: Vec<u64>,
}

#[snippet = "rolling_hash"]
impl RollingHash {
    pub fn new(s: &[u8], base: u64, modulo: u64) -> Self {
        let n = s.len();

        let mut power = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        for i in 0..n {
            power[i + 1] = power[i] * base % modulo;
            hash[i + 1] = (hash[i] * base + s[i] as u64) % modulo;
        }
        RollingHash {
            modulo: modulo,
            power: power,
            hash: hash,
        }
    }

    /// [l, r)
    pub fn get(&self, l: usize, r: usize) -> u64 {
        (self.hash[r] + self.modulo - (self.hash[l] * self.power[r - l]) % self.modulo)
            % self.modulo
    }
}

#[test]
fn test_rolling_hash() {
    use rand::Rng;
    use std::collections::HashMap;
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let mut chars = vec![];
        for _ in 0..100 {
            chars.push((rng.gen_range(0, 26) as u8 + 'a' as u8) as char);
        }

        let string = chars.iter().collect::<String>();
        let n = string.len();

        let rh = RollingHash2::new(string.as_bytes());
        let mut seen = HashMap::new();
        for i in 0..n {
            for len in 1..n + 1 {
                if i + len > n {
                    break;
                }
                let hash = rh.get(i, i + len);
                if seen.contains_key(&hash) {
                    assert_eq!(seen[&hash], &chars[i..i + len]);
                } else {
                    seen.insert(hash, &chars[i..i + len]);
                }
            }
        }
    }
}
