use std::i32;

fn main() {
    let n = read::<u32>();
    let lines = read_vec2::<i32>(n);

    let mut prev_t = 0;
    let mut prev_x = 0;
    let mut prev_y = 0;

    for line in lines {
        let t = line[0];
        let diff_t = t - prev_t;
        let x = line[1];
        let y = line[2];
        let diff_xy = (prev_x - x).abs() + (prev_y - y).abs();

        if diff_xy > diff_t {
            println!("No");
            return ();
        }

        if (diff_t - diff_xy) % 2 != 0 {
            println!("No");
            return ();
        } else {
            prev_t = t;
            prev_x = x;
            prev_y = y;
        }
    }

    println!("Yes");
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
