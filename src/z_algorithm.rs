#[snippet = "z_algorithm"]
pub fn z_algorithm(s: &[char]) -> Vec<usize> {
    let n = s.len();
    let mut prefix = vec![0; n];
    prefix[0] = n;

    let mut i = 1;
    let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] {
            j += 1;
        }
        prefix[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }

        let mut k = 1;
        while i + k < n && k + prefix[k] < j {
            prefix[i + k] = prefix[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    prefix
}

#[test]
fn test_z_algorithm() {
    let a: Vec<_> = "abcabcabaa".chars().collect();
    let prefix = z_algorithm(&a);
    assert_eq!(prefix, [10, 0, 0, 5, 0, 0, 2, 0, 1, 1]);
}

#[test]
fn test_z_algorithm_random() {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let mut chars = vec![];
        for _ in 0..100 {
            chars.push((rng.gen_range(0, 26) as u8 + 'a' as u8) as char);
        }
        let string = chars.iter().collect::<String>();
        let n = string.len();
        let prefix = z_algorithm(&chars);

        for i in 0..n {
            let mut len = 0;
            for j in 0..n {
                if i + j >= n {
                    break;
                }
                if chars[j] == chars[i + j] {
                    len += 1;
                } else {
                    break;
                }
            }
            assert_eq!(prefix[i], len);
        }
    }
}
