// Kadane algorithm.
use std::cmp::max;
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

fn kadane(nums: &[i64]) -> i64 {
    let mut sum = nums[0];
    let mut ans = nums[0];
    for &x in &nums[1..] {
        sum = max(sum + x, x);
        ans = max(ans, sum);
    }
    ans
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: u32 = scan.token();
    let mut nums = Vec::new();
    for _ in 0..n {
        nums.push(scan.token::<i64>());
    }
    let ans = kadane(&nums);
    writeln!(out, "{}", ans).ok();
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
