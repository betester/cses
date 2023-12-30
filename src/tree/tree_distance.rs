use std::{cmp::max, collections::HashMap};

const MAX_SIZE: usize = 200001;

fn tree_diamater(
    graph: &HashMap<usize, Vec<usize>>,
    first_max: &mut [usize; MAX_SIZE],
    second_max: &mut [usize; MAX_SIZE],
    max_child: &mut [usize; MAX_SIZE],
    current: usize,
) {
    if let Some(val) = graph.get(&current) {
        for &child in val.iter() {
            tree_diamater(graph, first_max, second_max, max_child, child);
            if first_max[child] + 1 > first_max[current] {
                second_max[current] = first_max[current];
                first_max[current] = first_max[child] + 1;
                max_child[current] = child;
            } else if first_max[child] + 1 > second_max[current] {
                second_max[current] = first_max[child] + 1;
            }
        }
    }
}

fn update_child_distance(
    graph: &HashMap<usize, Vec<usize>>,
    first_max: &mut [usize; MAX_SIZE],
    second_max: &mut [usize; MAX_SIZE],
    max_child: &mut [usize; MAX_SIZE],
    current: usize,
) {
    if let Some(val) = graph.get(&current) {
        for &child in val.iter() {
            if max_child[current] != child {
                second_max[child] = first_max[child];
                first_max[child] = first_max[current] + 1;
                max_child[child] = current;
            } else if first_max[child] < second_max[current] + 1 {
                second_max[child] = first_max[child];
                first_max[child] = second_max[current] + 1;
                max_child[child] = current;
            } else {
                second_max[child] = max(second_max[child], second_max[current] + 1);
            }
        }
    }
}

pub fn main(n: usize, edges: Vec<(usize, usize)>) {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut first_max: [usize; MAX_SIZE] = [0; MAX_SIZE];
    let mut second_max: [usize; MAX_SIZE] = [0; MAX_SIZE];
    let mut max_child: [usize; MAX_SIZE] = [0; MAX_SIZE];

    // the root always starts with 1
    let root = 1;

    for edge in edges.iter() {
        graph.entry(edge.0).or_insert_with(Vec::new).push(edge.1);
    }

    tree_diamater(
        &graph,
        &mut first_max,
        &mut second_max,
        &mut max_child,
        root,
    );
    update_child_distance(
        &graph,
        &mut first_max,
        &mut second_max,
        &mut max_child,
        root,
    );

    for i in 1..(n + 1) {
        print!("{}", first_max[i])
    }
}
