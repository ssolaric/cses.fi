// Rust doesn't have a Multiset struct, so we need to use a BTreeMap (frequency counter) as a replacement.
// We need to keep track of the multiset length, which IS NOT the same as the BTreeMap length.
use std::collections::BTreeMap;
use std::io;
use std::ops::Bound::*;
use std::str;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Interval {
    end: usize,
    start: usize,
}

fn last_less_or_equal(ends: &BTreeMap<usize, i32>, target: usize) -> Option<(&usize, &i32)> {
    let mut before_ub = ends.range((Unbounded, Included(target)));
    before_ub.next_back()
}

fn decrement(ends: &mut BTreeMap<usize, i32>, key: usize) {
    let entry = ends.entry(key).or_insert(0);
    if *entry == 1 {
        ends.remove(&key);
    } else {
        *entry -= 1;
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: usize = scan.token();
    let k: usize = scan.token();
    let mut intervals = Vec::new();
    for _ in 0..n {
        let a: usize = scan.token();
        let b: usize = scan.token();
        intervals.push(Interval { start: a, end: b });
    }
    intervals.sort();

    let mut ends = BTreeMap::new();
    let mut ends_length = 0;
    let mut ans = 0;
    for interval in intervals {
        let before_ub = last_less_or_equal(&ends, interval.start);
        if let Some((&end, _freq)) = before_ub {
            decrement(&mut ends, end);
            *ends.entry(interval.end).or_insert(0) += 1;
            ans += 1;
        }
        else if ends_length < k {
            *ends.entry(interval.end).or_insert(0) += 1;
            ends_length += 1;
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
