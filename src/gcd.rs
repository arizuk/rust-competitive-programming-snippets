#[snippet = "gcd"]
#[snippet = "lcm"]
pub fn gcd(a: u64, b:u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
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