#[snippet = "graph_dfs"]
#[snippet = "graph_bfs"]
#[snippet = "graph_topological_sort"]
#[derive(Debug)]
pub struct Graph {
    n: usize,
    edges: Vec<Vec<usize>>,
}

#[snippet = "graph_dfs"]
#[derive(Debug)]
pub struct DFSState {
    seen: Vec<bool>,
}

#[snippet = "graph_dfs"]
impl Graph {
    pub fn dfs(&self, cur: usize, from: usize, state: &mut DFSState) {
        if self.n == from {}
        state.seen[cur] = true;
        for &to in self.edges[cur].iter() {
            if to != from {
                self.dfs(to, cur, state);
            }
        }
    }
}

#[snippet = "graph_bfs"]
impl Graph {
    pub fn bfs(&self, s: usize) -> Vec<i64> {
        use std::collections::VecDeque;

        let mut q = VecDeque::new();

        let mut dist = vec![-1; self.n];
        dist[s] = 0;

        q.push_back(s);

        while let Some(cur) = q.pop_front() {
            for &adj in self.edges[cur].iter() {
                if dist[adj] == -1 {
                    dist[adj] = dist[cur] + 1;
                    q.push_back(adj);
                }
            }
        }

        dist
    }
}

#[snippet = "graph_topological_sort"]
impl Graph {
    pub fn topological_sort(&self) -> Vec<usize> {
        let n = self.n;
        let edges = &self.edges;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let n = 4;
        let g = Graph {
            n: n,
            edges: vec![vec![1, 2], vec![2], vec![], vec![]],
        };
        let mut state = DFSState {
            seen: vec![false; n],
        };
        g.dfs(0, n, &mut state);
        assert_eq!(state.seen, [true, true, true, false]);
    }

    #[test]
    fn test_bfs() {
        let n = 4;
        let g = Graph {
            n: n,
            edges: vec![vec![1, 2], vec![2], vec![3], vec![]],
        };
        let dist = g.bfs(0);
        assert_eq!(dist, [0, 1, 1, 2]);
    }

    #[test]
    fn test_topological_sort() {
        let n = 5;
        let g = Graph {
            n: n,
            edges: vec![vec![1, 2], vec![3], vec![4], vec![4], vec![]],
        };
        let ans = g.topological_sort();
        assert_eq!(ans, [0, 1, 2, 3, 4]);
    }
}
