use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead, Write};
use std::{cmp, mem};

#[derive(Clone, Default)]
struct Graph {
    s: usize,
    t: usize,
    flow: usize,
    node_count: usize,
    neighbormap: HashMap<usize, Vec<(usize, usize)>>,
    edgemap: HashMap<(usize, usize), usize>,
}

impl std::ops::IndexMut<(Node, Node)> for Graph {
    fn index_mut(&mut self, (u, v): (Node, Node)) -> &mut Self::Output {
        self.edgemap
            .get_mut(&(u.0, v.0))
            .expect("Failed to find key in edgemap")
    }
}

impl std::ops::Index<(Node, Node)> for Graph {
    type Output = usize;

    fn index(&self, (u, v): (Node, Node)) -> &Self::Output {
        &self.edgemap[&(u.0, v.0)]
    }
}

impl std::ops::IndexMut<Edge> for Graph {
    fn index_mut(&mut self, e: Edge) -> &mut Self::Output {
        self.edgemap
            .get_mut(&(e.src.0, e.dst.0))
            .expect("Failed to find key in edgemap")
    }
}

impl std::ops::Index<Edge> for Graph {
    type Output = usize;

    fn index(&self, e: Edge) -> &Self::Output {
        &self.edgemap[&(e.src.0, e.dst.0)]
    }
}

impl std::ops::Index<&Edge> for Graph {
    type Output = usize;

    fn index(&self, e: &Edge) -> &Self::Output {
        &self.edgemap[&(e.src.0, e.dst.0)]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Node(usize);

#[derive(Debug, PartialEq, Eq)]
struct EdgeMut<'a> {
    u: Node,
    v: Node,
    val: &'a mut usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Edge {
    src: Node,
    dst: Node,
}

impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.src.0, self.dst.0)
    }
}

impl Edge {
    fn rev(mut self) -> Self {
        mem::swap(&mut self.src, &mut self.dst);
        self
    }
}

impl Graph {
    fn new() -> Self {
        Self {
            s: 0,
            t: 0,
            flow: 0,
            node_count: 0,
            edgemap: HashMap::default(),
            neighbormap: HashMap::default(),
        }
    }
    fn make_undirected(&mut self) {
        let mut new_map = HashMap::new();
        for (&(u, v), &val) in self.edgemap.iter() {
            new_map.insert((u, v), val);
            new_map.insert((v, u), val);
        }
        self.edgemap = new_map;
    }

    fn set_source(&mut self, Node(s): Node) {
        self.s = s;
    }
    fn set_sink(&mut self, Node(t): Node) {
        self.t = t;
    }
    fn add_node(&mut self) -> Node {
        self.node_count += 1;
        Node(self.node_count)
    }
    fn add_edge(&mut self, Node(u): Node, Node(v): Node, val: usize) {
        let neighbors = self.neighbormap.entry(u).or_insert(vec![]);
        neighbors.push((u, v));
        self.edgemap.insert((u, v), val);
    }
    fn delete_edge(&mut self, Node(u): Node, Node(v): Node) {
        self.neighbormap
            .get_mut(&u)
            .map(|x| x.retain(|x| *x != (u, v)));
        self.edgemap.remove(&(u, v));
    }

    fn edges_mut<'a>(&'a mut self) -> impl Iterator<Item = EdgeMut<'a>> {
        self.edgemap.iter_mut().map(|(&(u, v), val)| EdgeMut {
            u: Node(u),
            v: Node(v),
            val,
        })
    }

    fn edges<'a>(&'a self) -> impl Iterator<Item = Edge> + 'a {
        self.edgemap.iter().map(|(&(u, v), _)| Edge {
            src: Node(u),
            dst: Node(v),
        })
    }

    fn neighbors<'a>(&'a self, Node(n): Node) -> impl Iterator<Item = Edge> + 'a {
        static EMPTY: Vec<(usize, usize)> = Vec::new();
        self.neighbormap
            .get(&n)
            .unwrap_or(&EMPTY)
            .iter()
            .map(|&(u, v)| Edge {
                src: Node(u),
                dst: Node(v),
            })
    }

    #[allow(dead_code)]
    fn print<W: Write>(&self, mut out: W, print_flow: bool) -> io::Result<()> {
        let Self {
            s,
            t,
            node_count,
            flow,
            edgemap: nodes,
            ..
        } = self;
        let flow = if print_flow {
            format!("{flow}")
        } else {
            String::new()
        };
        let n_edges = if print_flow {
            nodes.iter().filter(|(_, v)| **v > 0).count()
        } else {
            nodes.len()
        };
        writeln!(
            &mut out,
            "{node_count}\n\
             {s} {t} {flow}\n\
             {n_edges}"
        )?;

        for ((u, v), val) in nodes.iter() {
            if *val == 0 {
                continue;
            }
            writeln!(out, "{u} {v} {val}")?;
        }
        out.flush()?;
        Ok(())
    }

    #[allow(dead_code)]
    fn print_mathematica(&self) {
        println!("Graph[{{");
        for ((u, v), val) in self.edgemap.iter() {
            let fix = move |&n| {
                if n == self.s {
                    format!("s")
                } else if n == self.t {
                    format!("t")
                } else {
                    format!("{n}")
                }
            };

            let u = fix(u);
            let v = fix(v);
            println!("Labeled[DirectedEdge[{u}, {v}], {val}], ");
        }
        println!("}}]");
    }

    #[allow(dead_code)]
    fn read<R: BufRead>(rdr: R) -> Option<Self> {
        let mut lines = LineReader {
            rdr,
            line: String::new(),
        };

        let node_count = lines.advance()()?;
        let (s, t, flow) = {
            let mut next_int = lines.advance();
            let s = next_int().expect("missing s");
            let t = next_int().expect("missing t");
            let flow = next_int().unwrap_or(0);
            (s, t, flow)
        };
        let n_edges = lines.advance()().expect("missing n edges");
        let mut g = Graph {
            s,
            t,
            node_count,
            flow,
            ..Default::default()
        };
        for _ in 0..n_edges {
            let mut next_int = lines.advance();
            let u = next_int().expect("missing u");
            let v = next_int().expect("missing v");
            let t = next_int().unwrap_or(0);
            g.add_edge(Node(u), Node(v), t);
        }
        Some(g)
    }

    fn contains(&self, e: Edge) -> bool {
        self.edgemap.contains_key(&(e.src.0, e.dst.0))
    }
}

fn solve_flow(c: &Graph) -> Graph {
    let mut c = c.clone();
    let s = Node(c.s);
    let t = Node(c.t);
    let mut f = c.clone();
    let mut cf = Graph::new();
    for e in f.edges_mut() {
        cf.add_edge(e.v, e.u, *e.val);
        *e.val = 0;
    }
    for edge in cf.edges_mut() {
        f.add_edge(edge.u, edge.v, *edge.val);
    }
    c.make_undirected();

    let mut q = VecDeque::new();
    let mut path = vec![None; c.node_count + 1];
    let mut flow = 0;

    loop {
        path.fill(None);
        q.clear();
        q.push_back(s);
        while !q.is_empty() && path[t.0].is_none() {
            let cur = q.pop_front().unwrap();
            for e in f.neighbors(cur) {
                if path[e.dst.0].is_none() && e.dst != s && f[e] < c[e] {
                    path[e.dst.0] = Some(e);
                    q.push_back(e.dst);
                }
            }
        }

        if path[t.0].is_some() {
            let mut df = usize::MAX;
            let mut cur = path[t.0];
            while let Some(e) = cur {
                df = cmp::min(df, c[e] - f[e]);
                cur = path[e.src.0];
            }
            let mut cur = path[t.0];
            while let Some(e) = cur {
                f[e] += df;
                f[e.rev()] -= df;
                cur = path[e.src.0];
            }
            flow += df;
        } else {
            break;
        }
    }

    f.flow = flow;

    for edge in cf.edges_mut() {
        f.delete_edge(edge.u, edge.v);
    }
    f
}

struct LineReader<R: BufRead> {
    rdr: R,
    line: String,
}
impl<R: BufRead> LineReader<R> {
    fn advance<'a>(&'a mut self) -> impl 'a + FnMut() -> Option<usize> {
        self.line.clear();
        self.rdr
            .read_line(&mut self.line)
            .expect("couldn't read line");
        let mut split = self.line.split_whitespace();
        move || -> Option<usize> { split.next()?.parse().ok() }
    }
}

fn read_matching_problem<R: BufRead>(input: R) -> (Graph, usize) {
    let mut lines = LineReader {
        rdr: input,
        line: String::new(),
    };
    let (x, y);
    {
        let mut next_int = lines.advance();
        x = next_int().expect("couldn't read x");
        y = next_int().expect("couldn't read y");
    }
    let n_edges = lines.advance()().expect("couldn't read edge count");

    let mut g = Graph::new();
    g.node_count = x + y;

    for _ in 0..n_edges {
        let mut next_int = lines.advance();
        let u = next_int().expect("couldn't read u");
        let v = next_int().expect("couldn't read v");
        g.add_edge(Node(u), Node(v), 0);
    }

    (g, x)
}

fn create_flow_problem(g: &Graph, x: usize) -> Graph {
    let mut flow_problem = g.clone();
    let y = g.node_count;
    let s = flow_problem.add_node();
    let t = flow_problem.add_node();
    flow_problem.set_source(s);
    flow_problem.set_sink(t);
    for edge in flow_problem.edges_mut() {
        *edge.val = 1;
    }
    for u in 1..=x {
        flow_problem.add_edge(s, Node(u), 1);
    }
    for v in (x + 1)..=y {
        flow_problem.add_edge(Node(v), t, 1);
    }
    flow_problem
}

fn to_matching_solution(mut g: Graph) -> Vec<(Node, Node)> {
    let mut matching = Vec::with_capacity(10000);
    let s = Node(g.s);
    let t = Node(g.t);
    for EdgeMut { u, v, val } in g.edges_mut() {
        if *val == 0 {
            continue;
        }
        if u == s || u == t {
            continue;
        }
        if v == s || v == t {
            continue;
        }
        matching.push((u, v))
    }
    matching
}

fn main() {
    run().expect("lol");
}

fn run() -> Option<()> {
    const MATCH: bool = false;
    const FLOW: bool = true;
    const COMBINE: bool = MATCH && FLOW;

    if MATCH && !FLOW {
        // matching only code

        let (match_g, match_x) = read_matching_problem(io::stdin().lock());
        let flow_problem = create_flow_problem(&match_g, match_x);
        flow_problem
            .print(io::stdout(), false)
            .expect("failed printing");
        let flow_solution = Graph::read(io::stdin().lock()).expect("failed to make flow solution");
        let solution = to_matching_solution(flow_solution);
        let x = match_x;
        let y = match_g.node_count - x;
        println!("{x} {y}");
        println!("{}", solution.len());
        for (Node(u), Node(v)) in solution {
            println!("{u} {v}");
        }
    } else if FLOW && !MATCH {
        // flow code
        let mut flow_problem = Graph::read(io::stdin().lock()).expect("failed to make flow solution");
        let fix_info = fix_flow_problem(&mut flow_problem);
        let mut solution = solve_flow(&flow_problem);
        unfix_flow_problem(&mut solution, fix_info);
        solution.print(io::stdout(), true).expect("failed printing");
    } else if COMBINE {
        let (match_g, match_x) = read_matching_problem(io::stdin().lock());
        // match_g.print(io::stdout(), false);
        let flow_problem = create_flow_problem(&match_g, match_x);
        // flow_problem.print(io::stdout(), true);
        let flow_solution = solve_flow(&flow_problem);
        // println!("============ flow solution ============");
        // flow_solution.print(io::stdout().lock(), true);
        let match_solution = to_matching_solution(flow_solution);
        let x = match_x;
        let y = match_g.node_count - x;
        // println!("============ match solution ============");
        println!("{x} {y}");
        println!("{}", match_solution.len());
        for (Node(u), Node(v)) in match_solution {
            println!("{u} {v}");
        }
    }

    Some(())
}

fn unfix_flow_problem(g: &mut Graph, fix_info: FixInfo) {
    let (og_count, to_fix, phantom) = fix_info;

    for edge in to_fix {
        g.add_edge(edge.src, edge.dst, 0);
    }

    for (_, e1, e2) in phantom {
        if g.contains(e2) {
            g[(e1.src, e2.dst)] = g[e2];
        }
        g.delete_edge(e1.src, e1.dst);
        g.delete_edge(e2.src, e2.dst);
    }
    g.node_count = og_count;
}

type FixInfo = (usize, Vec<Edge>, Vec<(Node, Edge, Edge)>);

fn fix_flow_problem(g: &mut Graph) -> FixInfo {
    let og_count = g.node_count;
    let mut to_fix = vec![];
    let mut skip = HashSet::new();

    for e in g.edges() {
        if g.contains(e.rev()) && !skip.contains(&e.rev()) {
            skip.insert(e);
            to_fix.push(e.rev());
        }
    }

    let mut fixinfo = vec![];

    for fix in &to_fix {
        let new = g.add_node();
        fixinfo.push((new, Edge { src: fix.src, dst: new }, Edge { src: new, dst: fix.dst }));
        g.add_edge(fix.src, new, g[fix]);
        g.add_edge(new, fix.dst, g[fix]);
        g.delete_edge(fix.src, fix.dst);
    }

    (og_count, to_fix, fixinfo)
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufWriter};

    use super::*;
    // const FLOW_IN: &'static str = include_str!("../test-materials/testfall/bfstest.indata");
    // const FLOW_OUT: &'static str = include_str!("../test-materials/testfall/bfstest.utdata");
    const FLOW_IN: &'static str = include_str!("flow_in.txt");
    const FLOW_OUT: &'static str = include_str!("flow_out.txt");

    #[test]
    fn test_example() {
        let reader = BufReader::new(FLOW_IN.as_bytes());
        let problem = Graph::read(reader).expect("couldn't read the flow problem");
        let solution = solve_flow(&problem);
        let mut out = BufWriter::new(Vec::new());
        solution.print(&mut out, true).expect("write failed");
        let out =
            String::from_utf8(out.into_inner().unwrap()).expect("Somehow printed invalid utf");
        println!("{out}\n{FLOW_OUT}");
        assert_eq!(&out, FLOW_OUT);
    }
}
