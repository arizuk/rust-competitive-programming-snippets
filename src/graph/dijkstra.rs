#[snippet = "graph_shortest_path"]
#[snippet = "dijkstra"]
pub mod dijkstra {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    type Dist = i64;
    pub const INF: Dist = 1 << 60;

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct Rev(Dist);

    impl Ord for Rev {
        fn cmp(&self, other: &Rev) -> Ordering {
            other.0.cmp(&self.0)
        }
    }

    impl PartialOrd for Rev {
        fn partial_cmp(&self, other: &Rev) -> Option<Ordering> {
            Some(other.0.cmp(&self.0))
        }
    }

    pub fn shortest_path(edges: &Vec<Vec<(usize, Dist)>>, s: usize) -> Vec<Dist> {
        let n = edges.len();
        let mut dist = vec![INF; n];
        let mut heap = BinaryHeap::new();
        heap.push((Rev(0), s));

        while let Some((Rev(cur_dist), cur)) = heap.pop() {
            if cur_dist >= dist[cur] {
                continue;
            }
            dist[cur] = cur_dist;

            for &(adj, adj_dist) in edges[cur].iter() {
                if cur_dist + adj_dist < dist[adj] {
                    heap.push((Rev(cur_dist + adj_dist), adj));
                }
            }
        }
        dist
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scanner::Scanner;
    use std::io::BufReader;
    // https://onlinejudge.u-aizu.ac.jp/problems/GRL_1_A

    struct Problem {
        r: usize,
        edges: Vec<Vec<(usize,i64)>>
    }

    fn new_problem(input: String) -> Problem {
        let r = BufReader::new(input.as_bytes());
        let mut sc = Scanner { reader: r };

        let v: usize = sc.read();
        let e: usize = sc.read();
        let r: usize = sc.read();

        let mut edges = vec![vec![]; v];
        for _ in 0..e {
            let s: usize = sc.read();
            let t: usize = sc.read();
            let d: i64 = sc.read();
            edges[s].push((t, d));
        }
        Problem { r, edges }
    }

    #[test]
    fn test_grl_1_a_1() {
        let input = "
        4 5 0
        0 1 1
        0 2 4
        1 2 2
        2 3 1
        1 3 5
        "
        .to_string();
        let prob = new_problem(input);
        let dist = dijkstra::shortest_path(&prob.edges, prob.r);
        assert_eq!(dist, [0, 1, 3, 4]);
    }

    #[test]
    fn test_grl_1_a_2() {
        let input = "
        4 6 1
        0 1 1
        0 2 4
        2 0 1
        1 2 2
        3 1 1
        3 2 5
        "
        .to_string();
        let prob = new_problem(input);
        let dist = dijkstra::shortest_path(&prob.edges, prob.r);
        assert_eq!(dist, [3, 0, 2, dijkstra::INF]);
    }
}
