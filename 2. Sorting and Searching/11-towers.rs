use std::io;
use std::str;

fn upper_bound(arr: &[i32], target: i32) -> usize {
    let n = arr.len();
    let mut lo = 0;
    let mut hi = n;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] > target {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn minimize_towers(arr: &[i32]) -> usize {
    let mut towers = Vec::new();
    for &x in arr {
        let ind = upper_bound(&towers, x);
        if ind == towers.len() {
            towers.push(x);
        } else {
            towers[ind] = x;
        }
    }
    towers.len()
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let _: usize = scan.token();
    let arr: Vec<i32> = scan.tokens();
    let ans = minimize_towers(&arr);
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
