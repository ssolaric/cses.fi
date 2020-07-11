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

fn is_valid(x: i64, y: i64, n: i64) -> bool {
    x >= 0 && x < n && y >= 0 && y < n
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: i64 = scan.token();
    let dr = [-2, -1, 1, 2, 2, 1, -1, -2];
    let dc = [1, 2, 2, 1, -1, -2, -2, -1];

    for k in 1..=n {
        let mut ans = 0i64;
        for row in 0..k {
            for col in 0..k {
                let mut attacked_positions = 0;
                for d in 0..8 {
                    let nrow = row + dr[d];
                    let ncol = col + dc[d];
                    if is_valid(nrow, ncol, k) {
                        attacked_positions += 1;
                    }
                }
                ans += k * k - attacked_positions - 1;
            }
        }
        ans /= 2;
        writeln!(out, "{}", ans).ok();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
