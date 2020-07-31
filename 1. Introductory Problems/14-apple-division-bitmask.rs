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
    let mut apples = Vec::new();
    let mut sum = 0;
    for _ in 0..n {
        let weight: i64 = scan.token();
        sum += weight;
        apples.push(weight);
    }

    let mut ans = std::i64::MAX;
    for i in 0..(1 << n) {
        let mut sum1 = 0;
        for j in 0..n {
            if i & (1 << j) > 0 {
                sum1 += apples[j];
            }
        }
        let sum2 = sum - sum1;
        ans = ans.min((sum1 - sum2).abs());
    }
    writeln!(out, "{}", ans).ok();
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
