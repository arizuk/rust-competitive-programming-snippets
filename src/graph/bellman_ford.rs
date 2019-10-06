#[snippet = "bellman_ford"]
pub mod bellman_ford {
    type Dist = i64;
    pub const INF: Dist = 1 << 60;

    pub fn shortest_path(edges: &Vec<Vec<(usize, Dist)>>, s: usize) -> (Vec<Dist>, Vec<bool>) {
        let n = edges.len();
        let mut dist = vec![INF; n];
        dist[s] = 0;

        for _ in 0..n {
            for v in 0..n {
                for &(to, cost) in edges[v].iter() {
                    if dist[v] == INF || dist[to] <= dist[v] + cost {
                        continue;
                    }
                    dist[to] = dist[v] + cost;
                }
            }
        }

        // Propagate negative to all other vertices
        let mut negative = vec![false; n];
        for _ in 0..n {
            for v in 0..n {
                for &(to, cost) in edges[v].iter() {
                    if dist[v] == INF {
                        continue;
                    }
                    if dist[to] > dist[v] + cost {
                        dist[to] = dist[v] + cost;
                        negative[to] = true;
                    }
                    if negative[v] {
                        negative[to] = true;
                    }
                }
            }
        }
        return (dist, negative);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;
    use std::io::BufReader;
    // https://onlinejudge.u-aizu.ac.jp/problems/GRL_1_B

    struct Problem {
        r: usize,
        edges: Vec<Vec<(usize, i64)>>,
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
    fn test_grl_1_b_1() {
        let input = "
            4 5 0
            0 1 2
            0 2 3
            1 2 -5
            1 3 1
            2 3 2
        "
        .to_string();
        let prob = new_problem(input);
        let (dist, neg) = bellman_ford::shortest_path(&prob.edges, prob.r);

        let has_negative_cycle = neg.into_iter().any(|v| v);
        assert!(!has_negative_cycle);

        assert_eq!(dist, [0, 2, -3, -1]);
    }

    #[test]
    fn test_grl_1_b_2() {
        let input = "
            4 6 0
            0 1 2
            0 2 3
            1 2 -5
            1 3 1
            2 3 2
            3 1 0
        "
        .to_string();
        let prob = new_problem(input);
        let (_, neg) = bellman_ford::shortest_path(&prob.edges, prob.r);

        let has_negative_cycle = neg.into_iter().any(|v| v);
        assert!(has_negative_cycle);
    }

    #[test]
    fn test_grl_1_b_3() {
        let input = "
            4 5 1
            0 1 2
            0 2 3
            1 2 -5
            1 3 1
            2 3 2
        "
        .to_string();
        let prob = new_problem(input);
        let (dist, neg) = bellman_ford::shortest_path(&prob.edges, prob.r);

        let has_negative_cycle = neg.into_iter().any(|v| v);
        assert!(!has_negative_cycle);

        assert_eq!(dist, [bellman_ford::INF, 0, -5, -3]);
    }
}
