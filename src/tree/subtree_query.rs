use std::collections::HashMap;

macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let mut $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().expect("Should match with the type"))
            .collect::<Vec<$type>>();
    };
}
fn update_bit(bit: &mut Vec<i32>, start: usize, delta: i32) {
    let mut i: i32 = start as i32;

    while i < bit.len() as i32 {
        bit[i as usize] += delta;
        i += i & (!i + 1);
    }
}

fn query_bit(bit: &mut Vec<i32>, idx: usize) -> i32 {
    let mut total: i32 = 0;
    let mut i: usize = idx;

    while i > 0 {
        total += bit[i];
        i -= i & (!i + 1);
    }

    return total;
}

fn dfs(
    tree: &HashMap<usize, Vec<usize>>,
    value: &Vec<usize>,
    start: &mut Vec<usize>,
    end: &mut Vec<usize>,
    bit: &mut Vec<i32>,
    clock: usize,
    prev: usize,
    current: usize,
) {
    start[current] = clock;
    let mut last_clock = clock + 1;
    update_bit(bit, last_clock, value[current - 1] as i32);

    if let Some(val) = tree.get(&current) {
        for &edge in val.iter() {
            if edge != prev {
                dfs(tree, value, start, end, bit, last_clock, current, edge);
                last_clock = end[edge];
            }
        }
    }

    end[current] = last_clock;
}

pub fn main() {
    read_vec!(specs as usize);

    let (n, q) = (specs[0], specs[1]);

    let mut tree: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut start: Vec<usize> = vec![0; n + 1];
    let mut end: Vec<usize> = vec![0; n + 1];
    let mut bit: Vec<i32> = vec![0; n + 1];
    read_vec!(values as usize);

    for _ in 0..(n - 1) {
        read_vec!(edge as usize);
        tree.entry(edge[0]).or_insert_with(Vec::new).push(edge[1]);
        tree.entry(edge[1]).or_insert_with(Vec::new).push(edge[0]);
    }
    dfs(&tree, &values, &mut start, &mut end, &mut bit, 0, 0, 1);

    for _ in 0..q {
        read_vec!(query as usize);
        if query[0] == 1 {
            update_bit(&mut bit, start[query[1]], -(values[query[1] - 1] as i32));
            values[query[1] - 1] = query[2];
            update_bit(&mut bit, start[query[1]], values[query[1] - 1] as i32);
        } else {
            println!(
                "{}",
                query_bit(&mut bit, end[query[1]]) - query_bit(&mut bit, start[query[1]])
            );
        }
    }
}
