use std::cmp::min;

fn main() {
    let a = read::<i32>();
    let b = read::<i32>();
    let c = read::<i32>();
    let x = read::<i32>();

    let mut count = 0;
    for num_500 in 0..min(a, x / 500) + 1 {
        for num_100 in 0..min(b, x / 100) + 1 {
            for num_50 in 0..min(c, x / 50) + 1 {
                if num_500 * 500 + num_100 * 100 + num_50 * 50 == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
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
