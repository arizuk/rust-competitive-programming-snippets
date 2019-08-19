struct Graph {
    n: usize,
    edges: Vec<Vec<usize>>,
}
#[derive(Debug)]
struct State {
    seen: Vec<bool>
}
impl Graph {
    fn dfs(&self, cur: usize, from: usize, state: &mut State) {
        if self.n == from {
        }
        state.seen[cur] = true;
        for &to in self.edges[cur].iter() {
            if to != from {
                self.dfs(to, cur, state);
            }
        }
    }
}
impl Graph {
    fn bfs(&self, s: usize) {
        use std::collections::VecDeque;

        let mut q = VecDeque::new();
        q.push_back((s,0));

        let mut dists = vec![-1; self.n];

        while let Some((cur, dist)) = q.pop_front() {
            if dists[cur] >= 0 {
                continue;
            }
            dists[cur] = dist;

            for &adj in self.edges[cur].iter() {
                q.push_back((adj,dist+1));
            }
        }

        println!("dists={:?}", dists);
    }
}

fn main() {
    let edges = vec![
        vec![1,2],
        vec![2],
        vec![3],
        vec![],
    ];
    let n = 4;
    let dfs = {
        Graph { n: n, edges: edges }
    };
    let mut state = {
        State { seen: vec![false; n] }
    };
    dfs.dfs(0, 3, &mut state);
    
    let ref seen = state.seen;
    assert_eq!(seen, &[true, true, true, true]);

    dfs.bfs(0);
}