use std::collections::BTreeMap;
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
    let s: String = scan.token();
    let n = s.len();
    let mut counter = BTreeMap::new();
    for c in s.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }
    if n % 2 == 0 {
        // All frequencies must be even.
        for (_, freq) in &counter {
            if freq % 2 != 0 {
                writeln!(out, "NO SOLUTION").ok();
                return;
            }
        }
        let mut ans = vec!['_'; n];
        let mut i = 0;
        let mut j = n - 1;
        for (&c, &freq) in &counter {
            let mut tmp = freq;
            while tmp > 0 {
                ans[i] = c;
                ans[j] = c;
                i += 1;
                j -= 1;
                tmp -= 2;
            }
        }
        let ans: String = ans.into_iter().collect();
        writeln!(out, "{}", ans).ok();
    } else {
        // There must be one odd frequency, all others must be even.
        let mut odd_counter = 0;
        let mut odd_char = '_';
        for (&c, freq) in &counter {
            if freq % 2 != 0 {
                odd_counter += 1;
                odd_char = c;
            }
            if odd_counter > 1 {
                writeln!(out, "NO SOLUTION").ok();
                return;
            }
        }
        let mut ans = vec!['_'; n];
        ans[n / 2] = odd_char;
        *counter.entry(odd_char).or_insert(0) -= 1;
        let mut i = 0;
        let mut j = n - 1;
        for (&c, &freq) in &counter {
            let mut tmp = freq;
            while tmp > 0 {
                ans[i] = c;
                ans[j] = c;
                i += 1;
                j -= 1;
                tmp -= 2;
            }
        }
        let ans: String = ans.into_iter().collect();
        writeln!(out, "{}", ans).ok();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
