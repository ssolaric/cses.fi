use std::io;
use std::str;

// pred(x): Is it possible to make all products in less or equal than x seconds?
fn pred(x: i64, num_products: i64, machines: &[i64]) -> bool {
    let mut total = 0;
    for m in machines {
        total += x / m;
    }
    total >= num_products
}

fn factory_machines(num_products: i64, machines: &[i64]) -> i64 {
    let mut lo = 0;
    let mut hi = machines.iter().min().unwrap() * num_products;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if pred(mid, num_products, machines) {
            hi = mid;
        }
        else {
            lo = mid + 1;
        }
    }
    lo
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let _: usize = scan.token();
    let num_products: i64 = scan.token();
    let machines: Vec<i64> = scan.tokens();
    let ans = factory_machines(num_products, &machines);
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
