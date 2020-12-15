// This is the activity selection problem.
use std::io;
use std::str;

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
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: usize = scan.token();
    let mut intervals = Vec::new();
    for _ in 0..n {
        let a: i32 = scan.token();
        let b: i32 = scan.token();
        intervals.push((a, b));
    }
    intervals.sort_by_key(|i| i.1);

    let mut ans = 0;
    let mut end = -1;
    for i in 0..n {
        if intervals[i].0 >= end {
            ans += 1;
            end = intervals[i].1;
        }
    }
    writeln!(out, "{}", ans).ok();
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
