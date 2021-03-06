#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};
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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let k: i64 = get();
    let n = get();
    let mut a = vec![0i64; n];
    let mut d = vec![0i64; n];
    for i in 0 .. n {
        a[i] = get();
        d[i] = get();
    }
    let mut tot = 0;
    // pass: #possible zochiku < k
    let mut pass = 0;
    let mut fail = 1 << 40;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut count = 0;
        for i in 0 .. n {
            if mid >= a[i] {
                count += (mid - a[i]) / d[i] + 1;
            }
        }
        if count < k {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let mut num = 0;
    for i in 0 .. n {
        if fail >= a[i] {
            let count = (fail - a[i]) / d[i] + 1;
            num += count;
            tot += a[i] * count + d[i] * (count * (count - 1) / 2);
        }
    }
    tot -= (num - k) * fail;
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
