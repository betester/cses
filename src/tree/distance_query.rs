use std::{
    cmp::{max, min},
    collections::HashMap,
    usize,
};

use crate::read_vec;

const MAX_SIZE: usize = 200001;

fn dfs(
    tree: &HashMap<usize, Vec<usize>>,
    euler_path: &mut Vec<usize>,
    height: &mut Vec<usize>,
    first: &mut [usize; MAX_SIZE],
    current: usize,
    prev: usize,
    current_height: usize,
) {
    euler_path.push(current);
    height.push(current_height);
    first[current] = euler_path.len() - 1;
    if let Some(val) = tree.get(&current) {
        for &edge in val.iter() {
            if edge != prev {
                dfs(
                    tree,
                    euler_path,
                    height,
                    first,
                    edge,
                    current,
                    current_height + 1,
                );

                euler_path.push(current);
                height.push(current_height)
            }
        }
    }
}

fn query_st(st: &Vec<usize>, a: usize, b: usize, lb: usize, ub: usize, idx: usize) -> usize {
    if a > b {
        return 1_000_000_000;
    } else if a == lb && b == ub {
        return st[idx];
    } else {
        let mp = (ub + lb) / 2;
        let left_val = query_st(st, a, min(b, mp), lb, mp, idx * 2 + 1);
        let right_val = query_st(st, max(a, mp + 1), b, mp + 1, ub, idx * 2 + 2);

        return min(left_val, right_val);
    }
}

fn create_st(st: &mut Vec<usize>, array: &Vec<usize>, lb: usize, ub: usize, idx: usize) {
    if lb == ub {
        st[idx] = array[lb];
    } else {
        let mp = (ub + lb) / 2;
        create_st(st, array, lb, mp, 2 * idx + 1);
        create_st(st, array, mp + 1, ub, 2 * idx + 2);
        st[idx] = min(st[2 * idx + 1], st[2 * idx + 2]);
    }
}

pub fn main() {
    read_vec!(size as usize);
    let (n, q) = (size[0], size[1]);

    let mut tree: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..(n - 1) {
        read_vec!(edge as usize);

        tree.entry(edge[0]).or_insert_with(Vec::new).push(edge[1]);
        tree.entry(edge[1]).or_insert_with(Vec::new).push(edge[0]);
    }

    let mut euler_path = Vec::new();
    let mut height = Vec::new();
    let mut first = [0; MAX_SIZE];

    dfs(&tree, &mut euler_path, &mut height, &mut first, 1, 0, 0);

    let mut st = vec![1_000_000_000; 4 * height.len()];
    create_st(&mut st, &height, 0, height.len() - 1, 0);

    for _ in 0..(q) {
        read_vec!(query as usize);
        let a = min(first[query[0]], first[query[1]]);
        let b = max(first[query[0]], first[query[1]]);

        let (a_height, b_height) = (height[a], height[b]);

        let lca_height = query_st(&st, a, b, 0, height.len() - 1, 0);
        println!("{}", a_height + b_height - 2 * lca_height);
    }
}
