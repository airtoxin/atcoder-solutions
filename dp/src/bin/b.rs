use std::i32::MAX;
use std::cmp::min;

fn main() {
    let line = read_vec::<usize>();
    let n = line[0];
    let k = line[1];
    let h_vec = read_vec::<i32>();
    let mut costs = vec![MAX; n as usize];
    costs[0] = 0;

    for i in 1..n {
        for ki in 1..k+1 {
            if (i as i32) - (ki as i32) < 0 { continue; }
            costs[i] = min(costs[i], costs[i - ki] + (h_vec[i] - h_vec[i - ki]).abs());
        }
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
