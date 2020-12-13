// This is the 4sum problem.
use std::collections::HashSet;
use std::collections::HashMap;
use std::io;
use std::str;

fn four_sum(n: usize, a: &[i64], target: i64) -> Option<(usize, usize, usize, usize)> {
    let mut map = HashMap::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let sum2 = a[i] + a[j];
            map.insert(sum2, (i, j));
        }
    }

    for i in 0..n {
        for j in (i + 1)..n {
            let partial = target - a[i] - a[j];
            if let Some(&(ind1, ind2)) = map.get(&partial) {
                // i, j, ind1 and ind2 must be all distinct
                let mut set = HashSet::new();
                set.insert(i);
                set.insert(j);
                set.insert(ind1);
                set.insert(ind2);
                if set.len() == 4 {
                    return Some((i + 1, j + 1, ind1 + 1, ind2 + 1));
                }
            }
        }
    }
    None
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: usize = scan.token();
    let target: i64 = scan.token();
    let a: Vec<i64> = scan.tokens();
    match four_sum(n, &a, target) {
        Some((i, j, k, l)) => writeln!(out, "{} {} {} {}", i, j, k, l).ok(),
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
