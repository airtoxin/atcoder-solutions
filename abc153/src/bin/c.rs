fn main() {
    let line = read_vec::<i64>();
    let _n = line[0];
    let k = line[1];
    let mut h_vec = read_vec::<i64>();

    h_vec.sort();
    h_vec.reverse();

    let mut iter = h_vec.iter();
    for _ in 0..k {
        iter.next();
    }

    println!("{}", iter.sum::<i64>());
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace().map(|e| e.parse().ok().unwrap()).collect()
}
