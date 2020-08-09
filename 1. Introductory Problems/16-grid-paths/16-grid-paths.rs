// NOTE: compile this with optimizations:
// rustc -C opt-level=3 16-grid-paths.rs
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

const DR: [i32; 4] = [-1, 0, 1, 0];
const DC: [i32; 4] = [0, 1, 0, -1];
const SIZE: usize = 7;

#[inline]
fn is_valid(row: i32, col: i32, visited: &[[bool; SIZE]]) -> bool {
    let size = SIZE as i32;
    row >= 0 && row < size && col >= 0 && col < size && !visited[row as usize][col as usize]
}

#[inline]
fn left(d: usize) -> usize {
    (d + 3) % 4
}

#[inline]
fn right(d: usize) -> usize {
    (d + 1) % 4
}

fn can_visit_both_left_and_right(
    row: i32,
    col: i32,
    visited: &mut [[bool; SIZE]],
    d: usize,
) -> bool {
    let forward_row = row + DR[d];
    let forward_col = col + DC[d];
    if is_valid(forward_row, forward_col, visited) {
        return false;
    }
    let ld = left(d);
    let rd = right(d);
    let left_row = row + DR[ld];
    let left_col = col + DC[ld];
    let right_row = row + DR[rd];
    let right_col = col + DC[rd];
    is_valid(left_row, left_col, visited) && is_valid(right_row, right_col, visited)
}

// This has to be a macro, because if it is not, then the function call stack would grow quickly,
// since there would be too many recursive calls in the count_paths function.
macro_rules! count_paths_aux {
    ($depth:ident, $row:ident, $col:ident, $directions:ident, $visited:ident, $d:ident) => {{
        let nrow = ($row as i32) + DR[$d];
        let ncol = ($col as i32) + DC[$d];
        if is_valid(nrow, ncol, $visited)
            && !can_visit_both_left_and_right(nrow, ncol, $visited, $d)
        {
            count_paths(
                $depth + 1,
                nrow as usize,
                ncol as usize,
                $directions,
                $visited,
            )
        } else {
            0
        }
    }};
}

fn count_paths(
    depth: usize,
    row: usize,
    col: usize,
    directions: &[Option<usize>],
    visited: &mut [[bool; SIZE]],
) -> u32 {
    if row == SIZE - 1 && col == 0 {
        return (depth == SIZE * SIZE - 1) as u32;
    }
    visited[row][col] = true;
    let mut ans = 0;
    if let Some(dir) = directions[depth] {
        ans += count_paths_aux!(depth, row, col, directions, visited, dir);
    } else {
        for dir in 0..4 {
            ans += count_paths_aux!(depth, row, col, directions, visited, dir);
        }
    }
    visited[row][col] = false;
    ans
}

fn get_dir(c: char) -> Option<usize> {
    match c {
        'U' => Some(0),
        'R' => Some(1),
        'D' => Some(2),
        'L' => Some(3),
        _ => None,
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let description: String = scan.token();
    let directions: Vec<Option<usize>> = description.chars().map(|c| get_dir(c)).collect();
    let mut visited = [[false; SIZE]; SIZE];
    let ans = count_paths(0, 0, 0, &directions, &mut visited);
    writeln!(out, "{}", ans).ok();
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
