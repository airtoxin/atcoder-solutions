fn main() {
    let _n = read::<i32>();
    let mut a_vec = read_vec::<i32>();
    a_vec.sort();
    a_vec.reverse();

    let mut alice_points = 0;
    let mut bob_points = 0;
    for (i, a) in a_vec.iter().enumerate() {
        if i % 2 == 0 {
            alice_points += *a;
        } else {
            bob_points += *a;
        }
    }

    println!("{}", alice_points - bob_points);
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
