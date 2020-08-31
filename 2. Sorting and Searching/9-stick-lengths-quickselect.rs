// Minimize sum of |l - A[i]|, for some integer value "l".
// This sum is minimum when l is the median of A.
use std::io;
use std::str;

fn partition(nums: &mut [i64], lo: usize, hi: usize) -> usize {
    let pos_pivot = (lo + hi) / 2;
    let pivot = nums[pos_pivot];
    nums.swap(pos_pivot, hi);
    let mut i = lo;
    for j in lo..hi {
        if nums[j] < pivot {
            nums.swap(i, j);
            i += 1;
        }
    }
    nums.swap(i, hi);
    i
}

// A quickselect implementation. It chooses the middle element as the pivot.
// This implementation exceeds the time limit for huge arrays, for instance [1, 1, ..., 1] (1 repeated 200000 times).
fn quickselect(nums: &mut [i64], lo: i64, hi: i64, k: i64) {
    if lo >= hi {
        return;
    }
    let p = partition(nums, lo as usize, hi as usize) as i64;
    if p == k {
        return;
    }
    else if p < k {
        quickselect(nums, p + 1, hi, k);
    }
    else {
        quickselect(nums, lo, p - 1, k);
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: i64 = scan.token();
    let mut sticks: Vec<i64> = scan.tokens();
    quickselect(&mut sticks, 0, n - 1, n / 2);
    let median = sticks[(n as usize) / 2];
    let mut ans = 0;
    for x in sticks {
        ans += (median - x).abs();
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
