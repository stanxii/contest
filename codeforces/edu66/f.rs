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

// I read the editorial.
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut rnd = vec![vec![0u64; n + 1]; 2];
    {
        use std::hash::{Hasher, BuildHasher};
        let a = 0xdead_c0de_0013_3331;
        let b = 2457;
        let hm: HashMap<i32, i32> = HashMap::new();
        let mut hash = hm.hasher().build_hasher();
        hash.write_u32(8128);
        let mut x: u64 = hash.finish();
        for j in 0..2 {
            for i in 1..n + 1 {
                x = x.wrapping_mul(a).wrapping_add(b);
                rnd[j][i] = x ^ x << 10;
            }
        }
    }
    let mut acc = vec![[0; 2]; n + 1];
    let mut accpre = vec![[0; 2]; n + 2];
    for i in 0..n {
        for j in 0..2 {
            acc[i + 1][j] = acc[i][j] ^ rnd[j][a[i]];
        }
    }
    for i in 0..n + 1 {
        for j in 0..2 {
            accpre[i + 1][j] = accpre[i][j] ^ rnd[j][i];
        }
    }
    let mut tot: i64 = 0;
    let mut curmax = 1;

    let mut a = a;
    for _ in 0..2 {
        for i in 0..n {
            if a[i] == 1 {
                curmax = 1;
            } else {
                curmax = max(curmax, a[i]);
            }
            if i + 1 >= curmax {
                let t = [acc[i + 1][0] ^ acc[i + 1 - curmax][0],
                         acc[i + 1][1] ^ acc[i + 1 - curmax][1]];
                if accpre[curmax + 1] == t {
                    tot += 1;
                }
            }
        }
        a.reverse();
        acc.reverse();
    }
    for i in 0..n {
        if a[i] == 1 {
            tot -= 1;
        }
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
