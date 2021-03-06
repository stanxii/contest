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

fn dfs(v: usize, g: &[Vec<usize>], vis: &mut [bool], col: &mut [bool], c: bool) -> Result<(usize, usize), ()> {
    if vis[v] {
        return if c == col[v] {
            Ok((0, 0))
        } else {
            Err(())
        };
    }
    vis[v] = true;
    col[v] = c;
    let mut me = 1;
    let mut you = 0;
    for &w in &g[v] {
        let (x, y) = dfs(w, g, vis, col, !c)?;
        you += x;
        me += y;
    }
    Ok((me, you))
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        a: [usize1; n],
    }
    let mut g = vec![vec![]; n];
    for i in 0..n {
        g[a[i]].push(i);
        g[i].push(a[i]);
    }
    let mut tot = 0;
    let mut vis = vec![false; n];
    let mut col = vec![false; n];
    let mut ok = true;
    for i in 0..n {
        if !vis[i] {
            match dfs(i, &g, &mut vis, &mut col, false) {
                Ok((x, y)) => tot += max(x, y),
                Err(()) => ok = false,
            }
        }
    }
    puts!("{}\n", if ok { tot as i64 } else { -1 });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
