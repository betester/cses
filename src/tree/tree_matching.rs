use std::{cmp::max, collections::HashMap};

const MAX_SIZE: usize = 200001;

fn dfs(dp: &mut [[u32; 2]], graph: &HashMap<usize, Vec<usize>>, prev: usize, next: usize) -> u32 {
    if let Some(edges) = graph.get(&next) {
        // not taking any edges
        for &edge in edges.iter() {
            if edge != prev {
                dfs(dp, graph, next, edge);
                dp[next][0] += max(dp[edge][0], dp[edge][1]);
            }
        }

        // taking one edge
        for &edge in edges.iter() {
            if edge != prev {
                dp[next][1] = max(
                    dp[next][1],
                    dp[edge][0] + 1 + dp[next][0] - max(dp[edge][0], dp[edge][1]),
                );
            }
        }
    }

    return max(dp[1][0], dp[1][1]);
}

pub fn main(edges: Vec<(usize, usize)>) {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut dp: [[u32; 2]; MAX_SIZE] = [[0; 2]; MAX_SIZE];
    for edge in edges.iter() {
        graph.entry(edge.0).or_insert_with(Vec::new).push(edge.1);
        graph.entry(edge.1).or_insert_with(Vec::new).push(edge.0);
    }
    println!("{}", dfs(&mut dp, &graph, 0, 1));
}
