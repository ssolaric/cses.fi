// Rust doesn't have a Multiset struct, so we need to use a BTreeMap (frequency counter) as a replacement.
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io;
use std::str;

fn neighbors(tree: &BTreeSet<i32>, val: i32) -> (Option<&i32>, Option<&i32>) {
    use std::ops::Bound::*;
    let mut before = tree.range((Unbounded, Excluded(val)));
    let mut after = tree.range((Excluded(val), Unbounded));
    (before.next_back(), after.next())
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let x: i32 = scan.token();
    let n: i32 = scan.token();

    // Map segment length to the number of times this length appears in the array.
    let mut lengths: BTreeMap<i32, i32> = [(x, 1)].iter().cloned().collect();
    let mut positions: BTreeSet<_> = [0, x].iter().cloned().collect();
    for _ in 0..n {
        let position: i32 = scan.token();
        let (prev, next) = neighbors(&positions, position);
        let prev = prev.unwrap();
        let next = next.unwrap();

        let cur_length = next - prev;
        let new_length1 = position - prev;
        let new_length2 = next - position;
        let length_freq = lengths.entry(cur_length).or_insert(0);
        if *length_freq == 1 {
            lengths.remove(&cur_length);
        } else {
            *length_freq -= 1;
        }
        *lengths.entry(new_length1).or_insert(0) += 1;
        *lengths.entry(new_length2).or_insert(0) += 1;
        positions.insert(position);

        let (max_length, _) = lengths.iter().next_back().unwrap();
        write!(out, "{} ", max_length).ok();
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
