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
    let m: usize = scan.token();
    let k: i64 = scan.token();

    let mut intervals = Vec::new();
    for _ in 0..n {
        let desired_size: i64 = scan.token();
        intervals.push((desired_size - k, desired_size + k));
    }
    intervals.sort();

    let mut apartment_sizes = Vec::new();
    for _ in 0..m {
        apartment_sizes.push(scan.token::<i64>());
    }
    apartment_sizes.sort();

    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;
    while i < n && j < m {
        if apartment_sizes[j] < intervals[i].0 {
            j += 1;
        } else if intervals[i].0 <= apartment_sizes[j] && apartment_sizes[j] <= intervals[i].1 {
            ans += 1;
            i += 1;
            j += 1;
        } else {
            i += 1;
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
