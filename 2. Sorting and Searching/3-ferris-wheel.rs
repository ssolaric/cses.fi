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
    let x: i64 = scan.token();

    let mut weights = Vec::new();
    for _ in 0..n {
        weights.push(scan.token::<i64>());
    }
    weights.sort();

    let mut i = 0;
    let mut j = n - 1;
    let mut ans = 0;
    while i < j {
        let weight_sum = weights[i] + weights[j];
        if weight_sum <= x {
            i += 1;
            j -= 1;
        } else {
            j -= 1;
        }
        ans += 1;
    }
    if i == j {
        ans += 1;
    }
    writeln!(out, "{}", ans).ok();
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
