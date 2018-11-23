#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        a: [chars; n],
    }
    let mut ans = vec![vec![vec![0; 52]; 52]; 52];
    let b: Vec<Vec<usize>> = a.into_iter().map(|s| {
        s.into_iter().map(|c| {
            if c <= 'Z' {
                (c as u8 - b'A') as usize
            } else {
                (c as u8 - b'a') as usize + 26
            }
        }).collect()
    }).collect();
    const INF: usize = 1 << 27;
    let mut acc = vec![Vec::new(); n];
    for i in 0 .. n {
        let len = b[i].len();
        acc[i] = vec![vec![INF; 52]; len + 1];
        for j in (0 .. len).rev() {
            for k in 0 .. 52 {
                acc[i][j][k] = acc[i][j + 1][k];
            }
            acc[i][j][b[i][j]] = j;
        }
    }
    for i in 0 .. 52 {
        for j in 0 .. 52 {
            for u in 0 .. n {
                let st = acc[u][0][i];
                if st >= INF { continue; }
                let st2 = acc[u][st + 1][j];
                if st2 >= INF { continue; }
                for k in 0 .. 52 {
                    ans[i][j][k] += if acc[u][st2 + 1][k] < INF { 1 } else { 0 };
                }
            }
        }
    }
    let mut ma = 0;
    for i in 0 .. 52 {
        for j in 0 .. 52 {
            ma = max(ma, *ans[i][j].iter().max().unwrap());
        }
    }
    for i in 0 .. 52 {
        for j in 0 .. 52 {
            for k in 0 .. 52 {
                if ans[i][j][k] == ma {
                    for &c in &[i, j, k] {
                        if c >= 26 {
                            puts!("{}", ((c - 26) as u8 + b'a') as char);
                        } else {
                            puts!("{}", (c as u8 + b'A') as char);
                        }
                    }
                    puts!("\n");
                    return;
                }
            }
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
