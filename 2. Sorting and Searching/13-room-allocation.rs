// Similar to https://leetcode.com/problems/meeting-rooms-ii/
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;
use std::str;

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: usize = scan.token();
    let mut events = Vec::new();
    let mut rooms = vec![-1; n];
    for i in 0..n {
        let a: i32 = scan.token();
        let b: i32 = scan.token();
        events.push((a, 1, i));
        events.push((b, -1, i));
    }

    // If two events occur at the same time, consider the "start of an interval (1)" event first.
    events.sort_by(|e1, e2| {
        if e1.0 != e2.0 {
            e1.0.cmp(&e2.0)
        } else {
            e2.1.cmp(&e1.1)
        }
    });
    let mut min_rooms = 0;
    let mut used_rooms = 0;
    for &event in &events {
        used_rooms += event.1;
        min_rooms = min_rooms.max(used_rooms);
    }

    let mut unassigned_rooms = BinaryHeap::new();
    for i in 1..=min_rooms {
        unassigned_rooms.push(Reverse(i));
    }

    for &(_, accumulator, ind) in &events {
        if accumulator == 1 {
            if let Some(Reverse(room)) = unassigned_rooms.pop() {
                rooms[ind] = room;
            }
        } else {
            unassigned_rooms.push(Reverse(rooms[ind]));
        }
    }
    writeln!(out, "{}", min_rooms).ok();
    writeln!(out, "{}", rooms.iter().map(|&room| room.to_string()).collect::<Vec<String>>().join(" ")).ok();
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
