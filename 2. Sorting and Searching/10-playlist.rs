// A Sliding Window problem, similar to https://leetcode.com/problems/longest-substring-without-repeating-characters/
use std::collections::HashMap;
use std::io;
use std::str;

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: usize = scan.token();
    let arr: Vec<i32> = scan.tokens();

    let mut counter = HashMap::new();
    let mut i = 0;
    let mut j = 0;
    let mut ans = 0;
    while j < n {
        // TODO: find a more idiomatic way to do this.
        *counter.entry(&arr[j]).or_insert(0) += 1;
        while *counter.entry(&arr[j]).or_insert(0) > 1 {
            *counter.entry(&arr[i]).or_insert(0) -= 1;
            i += 1;
        }
        ans = ans.max(j - i + 1);
        j += 1;
    }
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
