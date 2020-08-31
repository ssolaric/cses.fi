// Minimize sum of |l - A[i]|, for some integer value "l".
// This sum is minimum when l is the median of A.
use std::io;
use std::str;

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: usize = scan.token();
    let mut sticks: Vec<i64> = scan.tokens();
    // Unlike C++, Rust doesn't have a "nth_element" function in its standard library.
    // If Rust had that function, this problem could be solved in linear time.
    sticks.sort();
    let median = sticks[n / 2];
    let mut ans = 0;
    for x in sticks {
        ans += (median - x).abs();
    }
    writeln!(out, "{}", ans).ok();
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    pub fn tokens<T: str::FromStr>(&mut self) -> Vec<T> {
        assert!(self.buffer.is_empty());
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
            .split_whitespace()
            .map(|x| x.parse().ok().expect("Failed parse"))
            .collect()
    }
}
