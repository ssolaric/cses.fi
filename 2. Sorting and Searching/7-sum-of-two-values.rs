// This is the 2sum problem.
use std::collections::HashMap;
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

fn two_sum(values: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut to_index = HashMap::new();
    let n = values.len();
    for i in 0..n {
        let to_find = target - values[i];
        if let Some(ind) = to_index.get(&to_find) {
            return Some((*ind, i));
        }
        to_index.insert(values[i], i);
    }
    None
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: usize = scan.token();
    let target: i32 = scan.token();
    let mut values = Vec::new();
    for _ in 0..n {
        values.push(scan.token::<i32>());
    }
    match two_sum(&values, target) {
        Some((pos1, pos2)) => writeln!(out, "{} {}", pos1 + 1, pos2 + 1).ok(),
        None => writeln!(out, "IMPOSSIBLE").ok(),
    };
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
