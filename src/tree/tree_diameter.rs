use std::{
    collections::{HashMap, LinkedList},
    usize,
};

const MAX_SIZE: usize = 200001;

fn bfs(graph: &HashMap<usize, Vec<usize>>, source: usize) -> (usize, usize) {
    let mut queue: LinkedList<(usize, usize)> = LinkedList::new();
    let mut current: (usize, usize) = (source, 0);
    let mut visited: [bool; MAX_SIZE] = [false; MAX_SIZE];

    queue.push_back(current);

    while let Some(val) = queue.pop_front() {
        current = val;
        visited[val.0] = true;

        if let Some(edges) = graph.get(&val.0) {
            for edge in edges.iter() {
                if !visited[*edge] {
                    queue.push_back((*edge, val.1 + 1));
                }
            }
        }
    }

    return current;
}

pub fn main(edges: Vec<Vec<usize>>) {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for edge in edges.iter() {
        graph.entry(edge[0]).or_insert_with(Vec::new).push(edge[1]);
        graph.entry(edge[1]).or_insert_with(Vec::new).push(edge[0]);
    }

    let furthest_node = bfs(&graph, 1);
    let final_furthest_node = bfs(&graph, furthest_node.0);

    println!("{}", final_furthest_node.1);
}
