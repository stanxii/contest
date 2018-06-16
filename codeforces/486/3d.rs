#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn solve() {
    let n = get();
    let mut x: Vec<i64> = (0 .. n).map(|_| get()).collect();
    x.sort_unstable();
    let mut hs = HashSet::new();
    for elem in x.iter() {
        hs.insert(elem);
    }
    let mut ma = vec![x[0]];
    for &elem in x.iter() {
        for i in 0 .. 35 {
            let y = elem + (1 << i);
            if hs.contains(&y) {
                ma = vec![elem, y];
            }
        }
    }
    for &elem in x.iter() {
        for i in 0 .. 35 {
            let y = elem + (1 << i);
            let z = elem + (1 << (i + 1));
            if hs.contains(&y) && hs.contains(&z) {
                ma = vec![elem, y, z];
            }
        }
    }
    println!("{}", ma.len());
    for i in 0 .. ma.len() {
        print!("{}{}", ma[i], if i + 1 == ma.len() { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
