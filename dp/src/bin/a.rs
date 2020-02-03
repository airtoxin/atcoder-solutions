use std::cmp::min;
use std::i32::MAX;

fn main() {
    let n = read::<usize>();
    let h_vec = read_vec::<i32>();
    let mut costs = vec![MAX; n];
    costs[0] = 0;
    costs[1] = (h_vec[0] - h_vec[1]).abs();

    for i in 2..n {
        costs[i] = min(costs[i], costs[i - 1] + (h_vec[i - 1] - h_vec[i]).abs());
        costs[i] = min(costs[i], costs[i - 2] + (h_vec[i - 2] - h_vec[i]).abs());
    }

    println!("{}", costs[costs.len() - 1]);
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

fn sum_each_digit(mut n: i32) -> i32 {
    let mut sum = 0;

    while n != 0 {
        sum += n % 10;
        n = n / 10;
    }

    sum
}
