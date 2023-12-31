mod input;
mod tree;

fn main() {
    read_vec!(sizes as usize);
    read_vec!(bosses as usize);
    let (n, q) = (sizes[0], sizes[1]);
    let mut queries: Vec<Vec<usize>> = Vec::new();

    for _ in 0..q {
        read_vec!(query as usize);
        queries.push(query);
    }

    tree::company_query::main(n, q, bosses, queries);
}
