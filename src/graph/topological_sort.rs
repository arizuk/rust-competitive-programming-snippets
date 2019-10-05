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

#[test]
fn test_topological_sort() {
    let n = 5;
    let edges =  vec![vec![1, 2], vec![3], vec![4], vec![4], vec![]];
    let ans = topological_sort(&edges);
    assert_eq!(ans, [0, 1, 2, 3, 4]);
}