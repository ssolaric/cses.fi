use std::collections::BTreeMap;
use std::io;
use std::ops::Bound::*;
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
    let n: u32 = scan.token();
    let m: u32 = scan.token();

    let mut tickets = BTreeMap::new();
    for _ in 0..n {
        let ticket: i32 = scan.token();
        *tickets.entry(ticket).or_insert(0) += 1;
    }
    for _ in 0..m {
        let max_price: i32 = scan.token();
        // Find the last price less or equal than max_price.
        let mut price_range = tickets.range_mut((Unbounded, Included(max_price)));
        // I can't directly remove the price from the map because of a double mutable borrow.
        // In order to get around this, I use a boolean variable to remove an element after
        // using its frequency value.
        // TODO: find a more idiomatic way of doing this.
        let mut to_remove = false;
        let mut found_price = 0;
        if let Some((price, freq)) = price_range.next_back() {
            writeln!(out, "{}", price).ok();
            found_price = *price;
            if *freq > 0 {
                *freq -= 1;
            } 
            if *freq == 0 {
                to_remove = true;
            }
        } else {
            writeln!(out, "-1").ok();
        }
        if to_remove {
            tickets.remove(&found_price);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
