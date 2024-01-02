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
    let (n, m, k) = (specs[0], specs[1], specs[2]);

    read_vec!(applicants as i32);
    read_vec!(apartments as i32);

    applicants.sort();
    apartments.sort();

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut bought = 0;

    while i < n && j < m {
        let apartment_price = apartments[j];
        let bargain_price = applicants[i];

        if (bargain_price - (k as i32) <= apartment_price)
            && (apartment_price <= bargain_price + (k as i32))
        {
            i += 1;
            j += 1;
            bought += 1;
        } else if apartment_price < bargain_price {
            j += 1;
        } else {
            i += 1;
        }
    }

    println!("{}", bought);
}
