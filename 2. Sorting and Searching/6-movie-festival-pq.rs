// This is the activity selection problem.
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;
use std::str;

#[derive(Ord, Eq, PartialEq, PartialOrd)]
struct Interval {
    end: usize,
    start: usize,
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: usize = scan.token();
    let mut intervals = BinaryHeap::new();
    for _ in 0..n {
        let a: usize = scan.token();
        let b: usize = scan.token();
        intervals.push(Reverse(Interval { start: a, end: b }));
    }

    let mut ans = 0;
    let mut end = 0;

    while !intervals.is_empty() {
        let Reverse(top) = intervals.pop().unwrap();
        if top.start >= end {
            end = top.end;
            ans += 1;
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
