use std::cmp::min;

fn main() {
    let line = read_vec::<i32>();
    let n = line[0];
    let y = line[1];

    for num_10000 in (0..n + 1).rev() {
        for num_5000 in (0..n - num_10000 + 1).rev() {
            let num_1000 = n - num_10000 - num_5000;
            if num_10000 * 10000 + num_5000 * 5000 + num_1000 * 1000 == y {
                println!("{} {} {}", num_10000, num_5000, num_1000);
                return ();
            }
        }
    }

    println!("-1 -1 -1");
}

pub fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

pub fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
