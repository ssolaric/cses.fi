use std::io;
use std::str;

fn find_min_subarrays(a: &[i64], desired_max_sum: i64) -> i32 {
    let mut cur_sum = 0;
    let mut count = 1;
    for x in a {
        if cur_sum + *x <= desired_max_sum {
            cur_sum += *x;
        }
        else {
            cur_sum = *x;
            count += 1;
        }
    }
    count
}

fn pred(a: &[i64], desired_max_sum: i64, k: i32) -> bool {
    find_min_subarrays(a, desired_max_sum) <= k
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let _: i32 = scan.token();
    let k: i32 = scan.token();
    let a: Vec<i64> = scan.tokens();
    let mut lo = *a.iter().max().unwrap() as usize;
    let mut hi = 1000000000000000000usize;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if pred(&a, mid as i64, k) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    writeln!(out, "{}", lo).ok();
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
