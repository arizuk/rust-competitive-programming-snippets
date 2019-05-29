#[snippet = "binary_search"]
pub fn binary_search<T: Ord>(a: &Vec<T>, x: &T) -> Result<usize, usize> {
    use std::cmp::Ordering;
    let mut l = 0;
    let mut r = a.len();

    while l != r {
        let m = l + (r - l) / 2;
        match a[m].cmp(x) {
            Ordering::Less => l = m + 1,
            Ordering::Equal | Ordering::Greater => r = m,
        }
    }
    if l < a.len() && a[l].cmp(x) == Ordering::Equal {
        Ok(l)
    } else {
        Err(r)
    }
}

#[snippet = "lower_bound"]
pub fn lower_bound<T: Ord>(a: &Vec<T>, x: &T) -> usize {
    use std::cmp::Ordering;
    let mut l = 0;
    let mut r = a.len();

    while l != r {
        let m = l + (r - l) / 2;
        match a[m].cmp(x) {
            Ordering::Less => l = m + 1,
            Ordering::Equal | Ordering::Greater => r = m,
        }
    }
    l
}

#[snippet = "uppper_bound"]
pub fn uppper_bound<T: Ord>(a: &Vec<T>, x: &T) -> usize {
    use std::cmp::Ordering;
    let mut l = 0;
    let mut r = a.len();

    while l != r {
        let m = l + (r - l) / 2;
        match a[m].cmp(x) {
            Ordering::Less | Ordering::Equal => l = m + 1,
            Ordering::Greater => r = m,
        }
    }
    l
}

#[snippet = "binary_search_by"]
// [l, r]
pub fn binary_search_by<F>(mut l: usize, mut r: usize, f: &F) -> Option<usize>
where
    F: Fn(usize) -> bool,
{
    assert!(l <= r);
    r += 1;
    let r_bound = r;
    while r != l {
        let m = l + (r - l) / 2; // avoid overflow
        if f(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    if r == r_bound {
        None
    } else {
        Some(r)
    }
}

#[test]
fn test_binary_search() {
    let a = vec![1, 2, 4, 4, 7, 12, 54, 60];

    assert_eq!(binary_search(&a, &4), Ok(2));
    assert_eq!(binary_search(&a, &5), Err(4));

    assert_eq!(lower_bound(&a, &0), 0);
    assert_eq!(lower_bound(&a, &4), 2);
    assert_eq!(lower_bound(&a, &61), 8);

    assert_eq!(uppper_bound(&a, &0), 0);
    assert_eq!(uppper_bound(&a, &4), 4);
    assert_eq!(uppper_bound(&a, &61), 8);
}

#[test]
fn test_binary_search_by() {
    let f = |m| m * m >= 1000;
    let ans = binary_search_by(1, 1000, &f);
    assert_eq!(Some(32), ans);

    let ans = binary_search_by(1, 32, &f);
    assert_eq!(Some(32), ans);

    let ans = binary_search_by(1, 31, &f);
    assert_eq!(None, ans);

    let ans = binary_search_by(32, 100, &f);
    assert_eq!(Some(32), ans);

    let ans = binary_search_by(33, 100, &f);
    assert_eq!(Some(33), ans);
}
