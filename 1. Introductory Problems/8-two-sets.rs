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

fn build_first(n: usize, sum: usize, used: &mut [bool]) -> Vec<usize> {
    let mut cur_sum = 0;
    let mut ans = Vec::new();

    if n % 2 == 0 {
        let mut i = 1;
        let mut j = n;
        while i < j && cur_sum < sum {
            ans.push(i);
            ans.push(j);
            used[i] = true;
            used[j] = true;
            cur_sum += i + j;
            i += 1;
            j -= 1;
        }
    } else {
        let mut i = n / 2;
        let mut j = i + 1;
        ans.push(n);
        used[n] = true;
        cur_sum = n;
        while j < n && cur_sum < sum {
            ans.push(i);
            ans.push(j);
            used[i] = true;
            used[j] = true;
            cur_sum += i + j;
            i -= 1;
            j += 1;
        }
    }
    ans
}

fn build_second(n: usize, used: &[bool]) -> Vec<usize> {
    let mut ans = Vec::new();
    for i in 1..=n {
        if !used[i] {
            ans.push(i);
        }
    }
    ans
}

fn print_vec<W: io::Write>(out: &mut W, nums: &[usize]) {
    writeln!(out, "{}", nums.len()).ok();
    for x in nums {
        write!(out, "{} ", x).ok();
    }
    writeln!(out).ok();
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, mut out: &mut W) {
    let n: usize = scan.token();
    let sum = n * (n + 1) / 2;
    if sum % 2 == 0 {
        writeln!(out, "YES").ok();
        let mut used = vec![false; n + 1];
        let first_set = build_first(n, sum / 2, &mut used);
        print_vec(&mut out, &first_set);
        let second_set = build_second(n, &mut used);
        print_vec(&mut out, &second_set);
    } else {
        writeln!(out, "NO").ok();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
