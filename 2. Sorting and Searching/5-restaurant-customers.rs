// 1D Sweep line technique
// See https://cses.fi/book/book.pdf, chapter 30.
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

enum EventType {
    Start, End
}

struct Event {
    time: i32,
    event_type: EventType
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n: usize = scan.token();
    // Instead of saving the intervals as a vector of pairs, we save them
    // as "events".
    let mut events = Vec::new();
    for _ in 0..n {
        let a: i32 = scan.token();
        let b: i32 = scan.token();
        events.push(Event {
            time: a,
            event_type: EventType::Start
        });
        events.push(Event {
            time: b,
            event_type: EventType::End
        });
    }
    events.sort_by_key(|e| e.time);
    let mut count = 0;
    let mut ans = 0;
    for event in &events {
        match event.event_type {
            EventType::Start => count += 1,
            EventType::End => count -= 1,
        }
        ans = ans.max(count);
    }
    writeln!(out, "{}", ans).ok();
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
