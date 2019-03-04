pub fn combination(n: u64, mut k: u64) -> u64 {
    assert!(n >= k);
    if k * 2 > n {
        k = n - k
    }
    let mut ans = 1;
    for d in 1..k + 1 {
        ans *= n + 1 - d;
        ans /= d;
    }
    ans
}

#[test]
fn test_combination() {
    assert_eq!(combination(4, 2), 6);
    assert_eq!(combination(4, 3), 4);
    assert_eq!(combination(4, 4), 1);
    combination(50, 25); // check overflow
}
