use std::collections::HashMap;
use std::io;
use std::str;

fn subarray_divisibility(a: &[i64]) -> i64 {
    let n = a.len() as i64;
    let mut counter = HashMap::new();
    counter.insert(0, 1);
    let mut sum = 0;
    let mut ans = 0;
    for x in a {
        sum = (sum + x).rem_euclid(n);
        ans += *counter.entry(sum).or_insert(0);
        *counter.entry(sum).or_insert(0) += 1;
    }
    ans
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let _: i32 = scan.token();
    let a: Vec<i64> = scan.tokens();
    let ans = subarray_divisibility(&a);
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
