mod input;
mod tree;

fn main() {
    read!(n as usize);
    let mut edges: Vec<Vec<usize>> = Vec::new();

    for _i in 0..(n - 1) {
        read_vec!(edge as usize);
        edges.push(edge);
    }

    tree::tree_diameter::main(edges);
}
