use std::collections::HashMap;

const MAX_SIZE: usize = 6;

fn total_node_distance(
    dp: &mut [usize; MAX_SIZE],
    graph: &HashMap<usize, Vec<usize>>,
    current: usize,
    prev: usize,
    current_distance: usize,
) -> usize {
    if let Some(val) = graph.get(&current) {
        for &edge in val.iter() {
            if edge != prev {
                dp[current] += 1
                    + current_distance
                    + total_node_distance(dp, graph, edge, current, current_distance + 1);
            }
        }
    }
    return dp[current];
}

pub fn main(edges: &Vec<Vec<usize>>) {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut dp: [usize; MAX_SIZE] = [0; MAX_SIZE];

    for edge in edges.iter() {
        graph.entry(edge[0]).or_insert_with(Vec::new).push(edge[1]);
        graph.entry(edge[1]).or_insert_with(Vec::new).push(edge[0]);
    }

    total_node_distance(&mut dp, &graph, 1, 0, 0);
    println!("{:?}", dp);
}
