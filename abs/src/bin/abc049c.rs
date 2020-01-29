//use regex::Regex;
//
//fn main() {
//    let s = read::<String>();
//    let re = Regex::new("^(dreamer|eraser|dream|erase)*$").unwrap();
//
//    println!("{}", if re.is_match(&s) { "YES" } else { "NO" });
//}

fn main() {
    let mut s = read::<String>();
    s = s.replace("eraser", "");
    s = s.replace("erase", "");
    s = s.replace("dreamer", "");
    s = s.replace("dream", "");

    println!("{}", if s.is_empty() { "YES" } else { "NO" });
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
