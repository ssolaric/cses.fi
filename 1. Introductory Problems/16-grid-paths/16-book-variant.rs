// See https://cses.fi/book/book.pdf, chapter 5.
// In a 7x7 grid, how many paths are there from the upper-left square to the lower-left square?
// Each path should traverse each square once.
// ans = 88418

// NOTE: compile this with optimizations:
// rustc -C opt-level=3 16-book-variant.rs

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

// Optimizations 3 and 4
// If the path cannot continue forward but can turn either left or right, the grid splits into two
// parts that both contain unvisited squares.
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
    ($depth:ident, $row:ident, $col:ident, $visited:ident, $d:ident) => {{
        let nrow = ($row as i32) + DR[$d];
        let ncol = ($col as i32) + DC[$d];
        if is_valid(nrow, ncol, $visited)
            && !can_visit_both_left_and_right(nrow, ncol, $visited, $d)
        {
            count_paths($depth + 1, nrow as usize, ncol as usize, $visited)
        } else {
            0
        }
    }};
}

fn count_paths(depth: usize, row: usize, col: usize, visited: &mut [[bool; SIZE]]) -> u32 {
    // Optimization 2
    if row == SIZE - 1 && col == 0 {
        return (depth == SIZE * SIZE - 1) as u32;
    }
    visited[row][col] = true;
    let mut ans = 0;
    for dir in 0..4 {
        ans += count_paths_aux!(depth, row, col, visited, dir);
    }
    visited[row][col] = false;
    ans
}

fn main() {
    let mut visited = [[false; SIZE]; SIZE];
    // We can't apply Optimization 1 for this variant of the original problem, because we no
    // longer have symmetry.
    let ans = count_paths(0, 0, 0, &mut visited);
    println!("{}", ans);
}
