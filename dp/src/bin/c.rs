use std::cmp::max;

fn main() {
    let n = read::<usize>();
    let abc_vec = read_vec2(n as u32);
    let mut values_abc = vec![vec![0; 3]; n];
    values_abc[0] = vec![abc_vec[0][0], abc_vec[0][1], abc_vec[0][2]];

    for i in 1..n {
        for abc_i in 0..3 {
            let v = abc_vec[i][(abc_i + 1) % 3];
            let w = abc_vec[i][(abc_i + 2) % 3];

            values_abc[i][abc_i] = max(values_abc[i][abc_i], values_abc[i - 1][abc_i] + max(v, w));
        }
    }

    let last = &values_abc[values_abc.len() - 1];
    println!("{:?}", values_abc);
    println!("{}", max(last[0], max(last[1], last[2])));
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
