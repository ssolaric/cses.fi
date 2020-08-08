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

const SIZE: usize = 8;

fn is_valid(row: usize, col: usize, board: &Vec<Vec<char>>, row_to_col: &[usize]) -> bool {
    if board[row][col] == '*' {
        return false;
    }
    for prev_row in 0..row {
        // Check column
        if row_to_col[prev_row] == col {
            return false;
        }
        // Check diagonals
        let delta_row = (row - prev_row) as i32;
        let delta_col = ((col as i32) - (row_to_col[prev_row] as i32)).abs();
        if delta_row == delta_col {
            return false;
        }
    }
    true
}

fn queens(depth: usize, board: &mut Vec<Vec<char>>, row_to_col: &mut [usize]) -> u32 {
    if depth == SIZE {
        return 1;
    }
    let mut ans = 0;
    for c in 0..SIZE {
        if is_valid(depth, c, board, row_to_col) {
            row_to_col[depth] = c;
            ans += queens(depth + 1, board, row_to_col);
            row_to_col[depth] = 0;
        }
    }
    ans
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let mut board = Vec::new();
    let mut row_to_col = vec![0; SIZE];
    for _ in 0..SIZE {
        let row: String = scan.token();
        let row: Vec<char> = row.chars().collect();
        board.push(row);
    }
    let ans = queens(0, &mut board, &mut row_to_col);
    writeln!(out, "{}", ans).ok();
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
