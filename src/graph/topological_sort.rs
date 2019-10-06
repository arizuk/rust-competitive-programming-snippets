#[snippet = "topological_sort"]
pub fn topological_sort(edges: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = edges.len();

    let mut h = vec![0; n];
    for i in 0..n {
        for &t in edges[i].iter() {
            h[t] += 1;
        }
    }

    use std::collections::VecDeque;
    let mut st = VecDeque::new();
    for i in 0..n {
        if h[i] == 0 {
            st.push_back(i);
        }
    }

    let mut sorted = vec![];
    while let Some(i) = st.pop_front() {
        sorted.push(i);
        for &t in edges[i].iter() {
            h[t] -= 1;
            if h[t] == 0 {
                st.push_back(t);
            }
        }
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use crate::scanner::Scanner;

    #[test]
    fn test_grl_4_b() {
        let input = "
            6 6
            0 1
            1 2
            3 1
            3 4
            4 5
            5 2
            ".to_string();

        let r = BufReader::new(input.as_bytes());
        let mut sc = Scanner { reader: r };
        let v:usize = sc.read();
        let e:usize = sc.read();

        let mut edges = vec![vec![]; v];
        for _ in 0..e {
            let s:usize = sc.read();
            let t:usize = sc.read();
            edges[s].push(t);
        }

        let ans = topological_sort(&edges);
        assert_eq!(ans, [0, 3, 1, 4, 5, 2]);
    }
}