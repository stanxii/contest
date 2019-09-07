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

fn calc(ans: &[usize]) -> i64 {
    let mut dp = vec![vec![0i64; 9]; ans.len() + 1];
    dp[0][0] = 1;
    for i in 0..ans.len() {
        for j in 0..9 {
            dp[i + 1][j] = dp[i + 1][j].checked_add(dp[i][j]).unwrap();
        }
        dp[i + 1][ans[i] + 1] = dp[i + 1][ans[i] + 1].checked_add(dp[i][ans[i]]).unwrap();
    }
    dp[ans.len()][8]
}

const W: usize = 501;
fn ex() -> Vec<i64> {
    let mut dp = vec![vec![0i64; W]; 8];
    dp[0] = vec![1; W];
    for i in 0..7 {
        for j in 1..W {
            dp[i + 1][j] = dp[i + 1][j - 1].saturating_add(dp[i][j]);
        }
    }
    debugln!("dp[7] = {:?}", dp[7]);
    dp[7].clone()
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        k: i64,
    }
    let tbl = ex();
    let mut ans = vec![];
    let mut rem = k;
    let mut freq = vec![0; W];
    for i in (1..W).rev() {
        let q = rem / tbl[i];
        freq[i] = q;
        rem -= tbl[i] * q;
    }
    for i in 1..W {
        for j in 0..7 {
            ans.push(j);
        }
        for _ in 0..freq[i] {
            ans.push(7);
        }
    }
    debugln!("{} == {}", k, calc(&ans));
    debugln!("ans.len() = {}", ans.len());
    let table: Vec<char> = "FESTIVAL".to_string().chars().collect();
    for idx in ans {
        puts!("{}", table[idx]);
    }
    puts!("\n");
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
