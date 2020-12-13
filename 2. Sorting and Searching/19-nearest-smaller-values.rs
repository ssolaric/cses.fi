// Monotonic stack
use std::io;
use std::str;

fn nearest_smaller_values(a: &[i32]) -> Vec<usize> {
    let n = a.len();
    let mut ans = vec![0; n];
    let mut stack = Vec::new();
    for i in (0..n).rev() {
        while !stack.is_empty() && a[i] < a[*stack.last().unwrap()] {
            let ind = *stack.last().unwrap();
            ans[ind] = i + 1;
            stack.pop();
        }
        stack.push(i);
    }
    ans
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let _: usize = scan.token();
    let a: Vec<i32> = scan.tokens();
    let ans = nearest_smaller_values(&a);
    for x in ans {
        write!(out, "{} ", x).ok();
    }
    writeln!(out).ok();
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
