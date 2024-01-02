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

pub fn main() {
    read_vec!(specs as usize);
    read_vec!(children as usize);

    let (n, x) = (specs[0], specs[1]);

    // sort, get min and max
    children.sort();

    let mut i = 0;
    let mut j = n - 1;
    let mut total_gondolas = 0;

    while i <= j {
        if children[i] + children[j] <= x {
            i += 1;
        }

        total_gondolas += 1;

        // either way, the j-th child won't fit in with the next i-th child
        j -= 1;
    }

    println!("{}", total_gondolas);
}
