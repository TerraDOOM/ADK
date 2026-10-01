use std::collections::HashMap;
use std::io;

#[derive(Clone, Default)]
struct Graph<T> {
    nodes: HashMap<(usize, usize), T>,
}

impl<T> Graph<T> {
    fn new() {
        Self::default()
    }
    fn add_edge(&mut self, e1: usize, e2: usize, val: T) {
        self.nodes.insert((e1, e1), val);
    }
}

fn main() {
    run().expect("lol");
}

    fn run() -> Option<()> {
        let stdin = io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s);
        let split = s.split_whitespace();
        let next_int = || {
            split
                .next()
                .map(|x| x.parse::<usize>().expect("failed to parse int"))
        };

        let m = next_int();
        let n = next_int();
        let edges = next_int();
    }
