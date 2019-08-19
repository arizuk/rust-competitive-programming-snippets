#[snippet = "graph_dfs"]
#[snippet = "graph_bfs"]
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
}
