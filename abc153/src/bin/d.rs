/*
1→1
==============+2
2→3
3→3
==============+4
4→7
5→7
6→7
7→7
==============+8
8→15
9→15
10→15
11→15
12→15
13→15
14→15
==============+16
15→31
*/

fn main() {
    let h = read::<i64>();

    let power_of_2s: Vec<i64> = (0..40).map(|i| 2i64.pow(i)).collect();

    let mut index = 0;
    while h > power_of_2s[0..index].into_iter().sum::<i64>() {
        index += 1;
    }

    println!("{}", power_of_2s[0..index].into_iter().sum::<i64>());
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

fn sum_each_digit(mut n: i32) -> i32 {
    let mut sum = 0;

    while n != 0 {
        sum += n % 10;
        n = n / 10;
    }

    sum
}
