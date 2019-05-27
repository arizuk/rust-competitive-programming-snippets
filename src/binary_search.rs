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
pub fn lower_bound<T: Ord>(a: &Vec<T>, x: &T) -> Option<usize> {
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
    if l == a.len() {
        None
    } else {
        Some(l)
    }
}

#[snippet = "uppper_bound"]
pub fn uppper_bound<T: Ord>(a: &Vec<T>, x: &T) -> Option<usize> {
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
    if l == a.len() {
        None
    } else {
        Some(l)
    }
}

#[test]
fn test_binary_search() {
    let a = vec![1, 2, 4, 4, 7, 12, 54, 60];

    assert_eq!(binary_search(&a, &4), Ok(2));
    assert_eq!(binary_search(&a, &5), Err(4));

    assert_eq!(lower_bound(&a, &0), Some(0));
    assert_eq!(lower_bound(&a, &4), Some(2));
    assert_eq!(lower_bound(&a, &61), None);

    assert_eq!(uppper_bound(&a, &0), Some(0));
    assert_eq!(uppper_bound(&a, &4), Some(4));
    assert_eq!(uppper_bound(&a, &61), None);
}
