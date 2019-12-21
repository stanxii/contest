#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
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

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize, l: i64,
        abc: [(usize1, usize1, i64); m],
        q: usize,
        st:[(usize1, usize1); q],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c) in &abc {
        if c <= l {
            g[a].push((b, c));
            g[b].push((a, c));
        }
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![vec![(INF, INF); n]; n];
    let mut dec = vec![vec![false; n]; n];
    for i in 0..n {
        dist[i][i] = (0, 0);
        dec[i][i] = false;
        loop {
            let mut mi = (INF, INF, 0);
            for j in 0..n {
                if !dec[i][j] {
                    mi = min(mi, (dist[i][j].0, dist[i][j].1, j));
                }
            }
            if mi.0 >= INF {
                break;
            }
            let v = mi.2;
            dec[i][v] = true;
            let (times, d) = dist[i][v];
            for &(w, c) in &g[v] {
                let mut nxt = (times, d + c);
                if d + c > l {
                    nxt = (times + 1, c);
                }
                dist[i][w] = min(dist[i][w], nxt);
            }
        }
    }
    for &(s, t) in &st {
        let ans = dist[s][t].0;
        puts!("{}\n", if ans >= INF { -1 } else { ans });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
