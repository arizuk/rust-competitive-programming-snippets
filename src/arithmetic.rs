#[snippet = "get_divisors"]
pub fn get_divisors(n: u64) -> Vec<u64> {
    let mut m = 1;
    let mut divs = vec![];
    while m * m <= n {
        if n % m == 0 {
            divs.push(m);
            if n / m != m {
                divs.push(n / m);
            }
        }
        m += 1;
    }
    divs.sort();
    divs
}

#[test]
fn test_get_divisors() {
    let divs = get_divisors(36);
    assert_eq!(divs, [1, 2, 3, 4, 6, 9, 12, 18, 36]);
}

#[snippet = "gcd"]
#[snippet = "lcm"]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet = "lcm"]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(10, 15), 5);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(10, 15), 30);
}
