use std::{
    cmp::{max, min},
    collections::HashMap,
};

const MAX_SIZE: usize = 200001;

macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().expect("Should match with the type"))
            .collect::<Vec<$type>>();
    };
}
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

fn dfs_for_answer(
    tree: &HashMap<usize, Vec<usize>>,
    values: &mut Vec<i32>,
    current: usize,
    prev: usize,
) {
    if let Some(val) = tree.get(&current) {
        for &edge in val.iter() {
            if edge != prev {
                dfs_for_answer(tree, values, edge, current);
                values[current] += values[edge];
            }
        }
    }
}

fn query_st(
    st: &Vec<Vec<usize>>,
    a: usize,
    b: usize,
    lb: usize,
    ub: usize,
    idx: usize,
) -> Vec<usize> {
    if a > b {
        return vec![1_000_000_000, 0];
    } else if a == lb && b == ub {
        return st[idx].clone();
    } else {
        let mp = (ub + lb) / 2;
        let left_val = query_st(st, a, min(b, mp), lb, mp, idx * 2 + 1);
        let right_val = query_st(st, max(a, mp + 1), b, mp + 1, ub, idx * 2 + 2);

        if left_val[0] < right_val[0] {
            return left_val;
        } else {
            return right_val;
        }
    }
}

fn create_st(st: &mut Vec<Vec<usize>>, array: &Vec<usize>, lb: usize, ub: usize, idx: usize) {
    if lb == ub {
        st[idx] = vec![array[lb], lb];
    } else {
        let mp = (ub + lb) / 2;
        create_st(st, array, lb, mp, 2 * idx + 1);
        create_st(st, array, mp + 1, ub, 2 * idx + 2);
        let left_val = &st[2 * idx + 1];
        let right_val = &st[2 * idx + 2];

        if left_val[0] < right_val[0] {
            st[idx] = left_val.clone();
        } else {
            st[idx] = right_val.clone();
        }
    }
}

pub fn main() {
    read_vec!(sizes as usize);
    let (n, q) = (sizes[0], sizes[1]);

    let mut tree: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut parent: Vec<usize> = vec![0; n + 1];
    let mut values: Vec<i32> = vec![0; n + 1];

    for _ in 0..(n - 1) {
        read_vec!(edge as usize);
        tree.entry(edge[0]).or_insert_with(Vec::new).push(edge[1]);
        tree.entry(edge[1]).or_insert_with(Vec::new).push(edge[0]);
        parent[edge[1]] = edge[0];
    }

    let mut euler_path = Vec::new();
    let mut height = Vec::new();
    let mut first = [0; MAX_SIZE];

    dfs(&tree, &mut euler_path, &mut height, &mut first, 1, 0, 0);

    let mut st = vec![vec![1_000_000_000, 0]; 4 * height.len()];
    create_st(&mut st, &height, 0, height.len() - 1, 0);

    for _ in 0..(q) {
        read_vec!(query as usize);
        let a = min(first[query[0]], first[query[1]]);
        let b = max(first[query[0]], first[query[1]]);

        //[height, idx]
        let lca_info = query_st(&st, a, b, 0, height.len() - 1, 0);
        let lca_node = euler_path[lca_info[1]];

        values[query[0]] += 1;
        values[query[1]] += 1;
        // to avoid counting twice on lca
        values[lca_node] -= 1;
        // to avoid calculating count coming from lca
        values[parent[lca_node]] -= 1;
    }
    dfs_for_answer(&tree, &mut values, 1, 0);

    for i in 1..values.len() {
        print!("{} ", values[i]);
    }
}
