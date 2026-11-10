use std::{
    cmp, collections::{BTreeSet, HashMap}, io::{self, Read, Write},
};

struct GraphProblem {
    v: u32,
    m: u32,
    edges: Vec<(u32, u32)>,
    disconnected: BTreeSet<u32>,
}

fn read_graph(input: &str) -> GraphProblem {
    let mut split = input.split_whitespace();
    let mut next = || {
        split
            .next()
            .expect("eof")
            .parse::<u32>()
            .expect("failed to parse u32")
    };
    let v = next();
    let mut unseen: BTreeSet<u32> = (1..=v).collect();
    let e = next();
    let m = next();
    let mut edges = Vec::with_capacity(e as usize);
    for _ in 0..e {
        let a = next();
        let b = next();
        if a == b { continue; }
        unseen.remove(&a);
        unseen.remove(&b);
        edges.push((a, b));
    }
    GraphProblem {
        v,
        m,
        edges,
        disconnected: unseen,
    }
}

// compress our graph problem to remove singlet nodes
fn compress(mut g: GraphProblem) -> GraphProblem {
    let mut mapping = HashMap::new();
    let set: BTreeSet<u32> = (1..=g.v).collect();
    let highest = set.difference(&g.disconnected);
    for (new, old) in g.disconnected.iter().zip(set.iter().rev()) {
        mapping.insert(*old, *new);
    }
    let map = |x| *mapping.get(&x).unwrap_or(&x);
    for e in &mut g.edges {
        e.0 = map(e.0);
        e.1 = map(e.1);
    }
    g.edges.retain(|(x, y)| x != y);
    g.v = g.v - g.disconnected.len() as u32;
    g.disconnected.clear();
    g
}

fn main() {
    let mut s = String::new();
    io::stdin()
        .read_to_string(&mut s)
        .expect("failed to read from stdin");
    let g = read_graph(&s);
    let g = compress(g);
    let k = cmp::min(g.m, 300) as usize;
    let mut role_line = Vec::with_capacity(match k {
        0..=9 => k * 2,
        10..=99 => k * 3,
        100..=300 => k * 4,
        _ => panic!("should never happen"),
    });
    for i in 2..=k {
        let _ = write!(&mut role_line, "{i} ");
    }
    let role_line = unsafe { String::from_utf8_unchecked(role_line) };
    println!("{}", g.v);
    println!("{}", g.edges.len());
    println!("{}", k);
    println!("1 1");
    println!("1 2");
    for _ in 3..=g.v {
        println!("{k} {role_line}");
    }
    for (a, b) in g.edges {
        if a == b { panic!(); }
        println!("2 {a} {b}");
    }
}
