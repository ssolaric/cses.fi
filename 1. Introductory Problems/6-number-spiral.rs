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

/*
right-up: odd layers
down-left: even layers
*/
fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let t: u64 = scan.token();
    for _ in 0..t {
        let row: u64 = scan.token();
        let col: u64 = scan.token();
        let ans: u64;
        let layer = row.max(col);
        // layer contains numbers from (layer - 1)**2 to layer**2
        if layer % 2 == 0 {
            // down-left layer
            if row == layer {
                // left
                ans = layer * layer - col + 1;
            } else {
                // down
                ans = (layer - 1) * (layer - 1) + row;
            }
        } else {
            // right-up layer
            if row == layer {
                // right
                ans = (layer - 1) * (layer - 1) + col;
            } else {
                // up
                ans = layer * layer - row + 1;
            }
        }
        writeln!(out, "{}", ans).ok();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
