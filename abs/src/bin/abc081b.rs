fn main() {
    let _n = read::<i32>();
    let mut a_vec = read_vec::<i32>();

    let mut count = 0;
    while a_vec.iter().all(|a| a % 2 == 0) {
        count += 1;
        a_vec = a_vec.iter().map(|a| a / 2).collect();
    }

    println!("{}", count);
}

pub fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace().map(|e| e.parse().ok().unwrap()).collect()
}

pub fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
