use std::collections::HashSet;

fn main() {
    let n = read::<i32>();
    let mut d_vec: Vec<i32> = vec![];
    for _ in 0..n {
        d_vec.push(read::<i32>());
    }
    let uniq_d: HashSet<i32> = d_vec.into_iter().collect();

    println!("{}", uniq_d.len());
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace().map(|e| e.parse().ok().unwrap()).collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
