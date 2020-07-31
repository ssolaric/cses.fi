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

fn knapsack(n: usize, w: usize, weights: &[i64], values: &[i64]) -> i64 {
    if n == 0 || w == 0 {
        return 0;
    }
    let mut ans = knapsack(n - 1, w, weights, values);
    let rem_w = (w as i64) - weights[n - 1];
    if rem_w >= 0 {
        ans = ans.max(knapsack(n - 1, rem_w as usize, weights, values) + values[n - 1]);
    }
    return ans;
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

    let sum1 = knapsack(n, (sum / 2) as usize, &apples, &apples);
    let sum2 = sum - sum1;
    writeln!(out, "{}", (sum1 - sum2).abs()).ok();
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
