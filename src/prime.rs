#[snippet = "generate_prime_table"]
pub fn generate_prime_table(n: u64) -> Vec<bool> {
    let mut is_prime = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n as usize {
        if is_prime[i] {
            let mut j = 2;
            while i * j <= n as usize {
                is_prime[i * j] = false;
                j += 1;
            }
        }
        i += 1;
    }
    is_prime
}

#[snippet = "generate_primes"]
pub fn generate_primes(n: u64) -> Vec<u64> {
    let is_prime = generate_prime_table(n);
    let mut primes = vec![];
    for i in 2..n + 1 {
        if is_prime[i as usize] {
            primes.push(i);
        }
    }
    primes
}

#[snippet = "prime_factorization"]
pub fn prime_factorization(mut n: usize) -> Vec<(usize, usize)> {
    let mut i = 2;
    let mut ans = vec![];
    while i * i <= n {
        let mut cnt = 0;
        while n % i == 0 {
            n /= i;
            cnt += 1;
        }
        if cnt > 0 {
            ans.push((i, cnt));
        }
        i += 1;
    }
    if n > 1 {
        ans.push((n, 1));
    }
    ans
}

#[test]
fn test_generate_prime_table() {
    assert_eq!(
        generate_prime_table(7),
        vec![false, false, true, true, false, true, false, true]
    );
}

#[test]
fn test_gen_primes() {
    assert_eq!(generate_primes(7), vec![2, 3, 5, 7]);
    assert_eq!(generate_primes(444777).len(), 37326);
}

#[test]
fn test_prime_factorization() {
    assert_eq!(prime_factorization(36), vec![(2, 2), (3, 2)]);
}
