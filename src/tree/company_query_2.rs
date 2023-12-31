use std::{
    cmp::{max, min},
    collections::HashMap,
};

use crate::{read, read_vec};

const MAX_SIZE: usize = 6;

fn dfs(
    tree: &HashMap<usize, Vec<usize>>,
    euler_path: &mut Vec<usize>,
    first: &mut [usize; MAX_SIZE],
    height: &mut Vec<usize>,
    root: usize,
    prev: usize,
    current_height: usize,
) {
    euler_path.push(root);
    height.push(current_height);
    first[root] = euler_path.len() - 1;
    if let Some(val) = tree.get(&root) {
        for &edge in val.iter() {
            if edge != prev {
                dfs(
                    tree,
                    euler_path,
                    first,
                    height,
                    edge,
                    root,
                    current_height + 1,
                );
                euler_path.push(root);
                height.push(current_height);
            }
        }
    }
}

fn construct_st(
    st: &mut Vec<(usize, usize)>,
    array: &Vec<usize>,
    lb: usize,
    ub: usize,
    idx: usize,
) {
    if ub == lb {
        st[idx] = (array[lb], lb);
    } else {
        let mp = (ub + lb) / 2;
        construct_st(st, array, lb, mp, 2 * idx + 1);
        construct_st(st, array, mp + 1, ub, 2 * idx + 2);

        let left_value = st[2 * idx + 1];
        let right_value = st[2 * idx + 2];

        st[idx] = if left_value.0 < right_value.0 {
            left_value
        } else {
            right_value
        };
    }
}

fn query_st(
    st: &Vec<(usize, usize)>,
    a: usize,
    b: usize,
    lb: usize,
    ub: usize,
    idx: usize,
) -> (usize, usize) {
    if a > b {
        (usize::MAX, 0)
    } else if a == lb && b == ub {
        st[idx]
    } else {
        let mp = (lb + ub) / 2;
        let left_value = query_st(st, a, min(b, mp), lb, mp, idx * 2 + 1);
        let right_value = query_st(st, max(a, mp + 1), b, mp + 1, ub, idx * 2 + 2);

        if left_value.0 < right_value.0 {
            left_value
        } else {
            right_value
        }
    }
}

pub fn main() {
    read!(n as usize);

    let mut tree: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..(n - 1) {
        read_vec!(edge as usize);
        tree.entry(edge[0]).or_insert_with(Vec::new).push(edge[1]);
        tree.entry(edge[1]).or_insert_with(Vec::new).push(edge[0]);
    }

    let mut euler_path: Vec<usize> = Vec::new();
    let mut height: Vec<usize> = Vec::new();
    let mut first: [usize; MAX_SIZE] = [0; MAX_SIZE];

    dfs(&mut tree, &mut euler_path, &mut first, &mut height, 1, 0, 0);

    let mut st: Vec<(usize, usize)> = vec![(usize::MAX, 0); 4 * height.len()];
    construct_st(&mut st, &height, 0, height.len() - 1, 0);
    //let (_, idx) = query_st(
    //    &st,
    //    min(first[2], first[5]),
    //    max(first[2], first[5]),
    //    0,
    //    height.len(),
    //    0,
    //);
    //println!("{}", euler_path[idx]);
}
