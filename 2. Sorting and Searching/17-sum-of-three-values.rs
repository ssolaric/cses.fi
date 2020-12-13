// This is the 3sum problem.
use std::io;
use std::str;

fn three_sum(n: usize, a: &[i64], target: i64) -> Option<(usize, usize, usize)> {
    let mut a_with_indices = Vec::new();
    for (ind, x) in a.iter().enumerate() {
        a_with_indices.push((x, ind));
    }
    a_with_indices.sort();
    for i in 0..n {
        let mut j = i + 1;
        let mut k = n - 1;
        let partial = target - a_with_indices[i].0;
        while j < k {
            if a_with_indices[j].0 + a_with_indices[k].0 == partial {
                return Some((a_with_indices[i].1 + 1, a_with_indices[j].1 + 1, a_with_indices[k].1 + 1));
            } else if a_with_indices[j].0 + a_with_indices[k].0 < partial {
                j += 1;
            } else {
                k -= 1;
            }
        }
    }
    None
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: usize = scan.token();
    let target: i64 = scan.token();
    let a: Vec<i64> = scan.tokens();
    match three_sum(n, &a, target) {
        Some((i, j, k)) => writeln!(out, "{} {} {}", i, j, k).ok(),
        None => writeln!(out, "IMPOSSIBLE").ok(),
    };
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
