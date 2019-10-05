// Copied from https://github.com/kenkoooo/competitive-programming-rs/blob/6b6ee7bb77f3402587f2c90e701aafdd82aec04b/src/graph/strongly_connected_components.rs
// Thanks kenkoooo!

#[snippet = "strongly_connected_components"]
pub mod strongly_connected_components {
    use std::collections::VecDeque;
    pub fn decompose(graph: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut vs = Vec::new();
        let num_v = graph.len();
        let mut cmp = vec![0; num_v];

        let mut reverse_graph = vec![vec![]; num_v];
        for i in 0..num_v {
            for &v in graph[i].iter() {
                reverse_graph[v].push(i);
            }
        }
        let mut used = vec![false; num_v];

        let mut stack = VecDeque::new();
        let mut added = vec![false; num_v];
        for i in 0..num_v {
            if used[i] {
                continue;
            }
            stack.push_front(i);
            while let Some(v) = stack.pop_front() {
                stack.push_front(v);
                used[v] = true;
                let mut pushed = false;
                for &u in graph[v].iter().rev() {
                    if !used[u] {
                        stack.push_front(u);
                        pushed = true;
                    }
                }
                if !pushed {
                    stack.pop_front();
                    if !added[v] {
                        vs.push(v);
                        added[v] = true;
                    }
                }
            }
        }

        used = vec![false; num_v];
        let mut k = 0;
        vs.reverse();
        for &i in vs.iter() {
            if used[i] {
                continue;
            }
            stack.push_front(i);
            used[i] = true;
            cmp[i] = k;
            while let Some(v) = stack.pop_front() {
                stack.push_front(v);
                let mut pushed = false;
                for &u in reverse_graph[v].iter() {
                    if used[u] {
                        continue;
                    }
                    used[u] = true;
                    cmp[u] = k;
                    stack.push_front(u);
                    pushed = true;
                }
                if !pushed {
                    stack.pop_front();
                }
            }
            k += 1;
        }

        return cmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let edges = vec![
            (1, 2),
            (1, 4),
            (2, 3),
            (3, 6),
            (4, 5),
            (4, 7),
            (5, 6),
            (6, 3),
            (7, 1),
        ];
        let n = 7;
        let mut graph = vec![vec![]; n];
        for (a, b) in edges {
            graph[a - 1].push(b - 1);
        }

        let classes = strongly_connected_components::decompose(&graph);
        let mut components = classes.clone();
        components.sort();
        components.dedup();
        assert_eq!(components.len(), 4);

        let expected = vec![vec![1, 4, 7], vec![2], vec![3, 6], vec![5]];
        for expected_nodes in expected {
            let expected_nodes: Vec<_> = expected_nodes.into_iter().map(|v| v - 1).collect();
            let current_class = classes[expected_nodes[0]];

            let mut nodes = vec![];
            for i in 0..n {
                if classes[i] == current_class {
                    nodes.push(i);
                }
            }
            assert_eq!(nodes, expected_nodes);
        }
    }
}
