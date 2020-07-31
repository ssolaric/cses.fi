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

fn backtrack(
    depth: usize,
    s: &[char],
    visited: &mut [bool],
    ans: &mut Vec<String>,
    partial: &mut Vec<char>,
) {
    let n = s.len();
    if depth == n {
        ans.push(partial.iter().collect());
        return;
    }
    for i in 0..n {
        if visited[i] {
            continue;
        }
        if i > 0 && s[i - 1] == s[i] && !visited[i - 1] {
            continue;
        }
        visited[i] = true;
        partial.push(s[i]);
        backtrack(depth + 1, s, visited, ans, partial);
        partial.pop().unwrap();
        visited[i] = false;
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let s: String = scan.token();
    let mut s: Vec<char> = s.chars().collect();
    s.sort();
    let n = s.len();
    let mut ans: Vec<String> = Vec::new();
    let mut partial: Vec<char> = Vec::new();
    let mut visited = vec![false; n];
    backtrack(0, &s, &mut visited, &mut ans, &mut partial);

    writeln!(out, "{}", ans.len()).ok();
    for st in ans {
        writeln!(out, "{}", st).ok();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
