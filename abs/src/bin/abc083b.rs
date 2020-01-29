fn main() {
    let line = read_vec::<i32>();
    let n = line[0];
    let a = line[1];
    let b = line[2];

    let mut sum = 0;
    for x in 1..n + 1 {
        let digit_sum = sum_each_digit(x);
        if a <= digit_sum && digit_sum <= b {
            sum += x;
        }
    }

    println!("{}", sum);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
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
