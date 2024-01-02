macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

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
    read!(n as usize);
    read_vec!(numbers as usize);

    numbers.sort();
    let mut current_number = numbers[0];
    let mut distinct_number = 1;
    for i in 1..n {
        if current_number != numbers[i] {
            distinct_number += 1;
        }
        current_number = numbers[i];
    }

    println!("{}", distinct_number);
}
