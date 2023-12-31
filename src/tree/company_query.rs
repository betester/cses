use std::{cmp::max, collections::HashMap};

use crate::read_vec;

fn get_tree_height(tree: &HashMap<usize, Vec<usize>>, root: usize, prev: usize) -> usize {
    let mut height = 0;
    if let Some(val) = tree.get(&root) {
        for &edge in val.iter() {
            if prev != edge {
                height = max(1 + get_tree_height(tree, edge, root), height);
            }
        }
    }

    return height;
}

fn find_parent(dp: &Vec<Vec<usize>>, i: usize, k: usize) -> usize {
    if k == 0 || i == 0 {
        return i;
    } else {
        let mut j = dp[i].len() - 1;
        while (1 << j) > k {
            j -= 1;
        }

        return find_parent(dp, dp[i][j], k - (1 << j));
    }
}

pub fn main(n: usize, q: usize, bosses: Vec<usize>, queries: Vec<Vec<usize>>) {
    let mut tree: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..(n - 1) {
        tree.entry(i + 2).or_insert_with(Vec::new).push(bosses[i]);
        tree.entry(bosses[i]).or_insert_with(Vec::new).push(i + 2);
    }

    let tree_height = (get_tree_height(&tree, 1, 0) as f32).log2().ceil() as usize;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; tree_height]; n + 1];

    for i in 2..(n + 1) {
        dp[i][0] = bosses[i - 2];
        for j in 1..(tree_height) {
            dp[i][j] = dp[dp[i][j - 1]][j - 1];
        }
    }

    for query in queries.iter() {
        println!("{}", find_parent(&dp, query[0], query[1]));
    }
}
