const MOD: usize = 1e9 as usize + 7;

#[snippet = "mod_division"]
#[allow(dead_code)]
pub struct ModDivision {
    fact: Vec<usize>, // 階乗
    inv: Vec<usize>,  // 逆元
    finv: Vec<usize>, // 階乗の逆元
}

#[snippet = "mod_division"]
impl ModDivision {
    pub fn new(max_value: usize) -> Self {
        let mut fact = vec![0; max_value + 1];
        let mut inv = vec![0; max_value + 1];
        let mut finv = vec![0; max_value + 1];

        fact[0] = 1;
        fact[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;

        for i in 2..max_value + 1 {
            fact[i] = fact[i - 1] * i % MOD;
            inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
            finv[i] = finv[i - 1] * inv[i] % MOD;
        }

        ModDivision {
            fact: fact,
            inv: inv,
            finv: finv,
        }
    }

    pub fn combination(&self, n: usize, k: usize) -> usize {
        assert!(n >= k);
        self.fact[n] * self.finv[n - k] % MOD * self.finv[k] % MOD
    }
}

#[snippet = "mod_pow"]
// b^p
pub fn mod_pow(b: usize, p: usize) -> usize {
    if p == 0 {
        return 1;
    }
    let mut ret = mod_pow(b * b % MOD, p / 2) % MOD;
    if p % 2 == 1 {
        ret = ret * b % MOD;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_division() {
        let div = ModDivision::new(10);
        assert_eq!(div.combination(5, 2), 10);
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 0), 1);
        assert_eq!(mod_pow(2, 1), 2);
        assert_eq!(mod_pow(2, 2), 4);
        assert_eq!(mod_pow(2, 10), 1024);
        assert_eq!(mod_pow(5, 34), 836844666);
    }
}
