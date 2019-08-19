const MOD: usize = 1e9 as usize + 7;

#[snippet = "mod_factorial"]
#[allow(dead_code)]
pub struct ModFactorial {
    fact: Vec<usize>, // 階乗
    inv: Vec<usize>,  // 逆元
    finv: Vec<usize>, // 階乗の逆元
    modulo: usize,
}

#[snippet = "mod_factorial"]
impl ModFactorial {
    pub fn new(max_value: usize, modulo: usize) -> Self {
        let mut fact = vec![0; max_value + 1];
        let mut inv = vec![0; max_value + 1];
        let mut finv = vec![0; max_value + 1];

        fact[0] = 1;
        fact[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;

        for i in 2..max_value + 1 {
            fact[i] = fact[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            finv[i] = finv[i - 1] * inv[i] % modulo;
        }

        ModFactorial {
            fact: fact,
            inv: inv,
            finv: finv,
            modulo: modulo,
        }
    }

    pub fn combination(&self, n: usize, k: usize) -> usize {
        assert!(n >= k);
        self.fact[n] * self.finv[n - k] % self.modulo * self.finv[k] % self.modulo
    }
}

#[snippet = "mod_op"]
pub struct ModOp {
    modulo: usize,
}

#[snippet = "mod_op"]
impl ModOp {
    pub fn new(modulo: usize) -> Self {
        ModOp { modulo: modulo }
    }

    /// b^power
    pub fn pow(&self, mut b: usize, mut power: usize) -> usize {
        let mut result = 1;
        while power > 0 {
            if power & 1 == 1 {
                result = (result * b) % self.modulo;
            }
            b = (b * b) % self.modulo;
            power >>= 1;
        }
        result
    }

    pub fn inv(&self, a: usize) -> usize {
        self.pow(a, self.modulo - 2)
    }
}

#[snippet = "mod_pow"]
/// b^power
pub fn mod_pow(mut b: usize, mut power: usize) -> usize {
    let mut result = 1;
    while power > 0 {
        if power & 1 == 1 {
            result = (result * b) % MOD;
        }
        b = (b * b) % MOD;
        power >>= 1;
    }
    result
}

#[snippet = "mod_pow"]
pub fn inv(a: usize) -> usize {
    mod_pow(a, MOD - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_factorial() {
        let mod_fact = ModFactorial::new(10, MOD);
        assert_eq!(mod_fact.combination(5, 2), 10);
    }

    #[test]
    fn test_mod_op() {
        let mod_op = ModOp::new(MOD);
        assert_eq!(mod_op.pow(2, 0), 1);
        assert_eq!(mod_op.pow(2, 1), 2);
        assert_eq!(mod_op.pow(2, 2), 4);
        assert_eq!(mod_op.pow(2, 10), 1024);
        assert_eq!(mod_op.pow(5, 34), 836844666);
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
