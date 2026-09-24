use std::{
    collections::BinaryHeap,
    fs::File,
    io::{self, Read},
    mem,
    ops::Range,
    sync::{LazyLock, RwLock},
    time::Instant,
};

#[derive(Copy, Clone)]
struct Lcg {
    state: u32,
}

impl Lcg {
    // taken from rosetta code
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.state %= 1 << 31;
        self.state
    }

    // biased but we don't really care for these purposes
    fn range(&mut self, r: Range<usize>) -> usize {
        let next = self.next_u32() as usize;
        next % (r.end - r.start) + r.start
    }
}

fn random_range(r: Range<usize>) -> usize {
    // SAFETY: this program isn't multithreaded, otherwise this would blow up
    static mut RAND: Lcg = Lcg { state: 0 };
    let ptr = &raw mut RAND;
    let mut lcg = unsafe { *ptr };
    let rand = lcg.range(r);
    unsafe { *ptr = lcg };
    rand
}

struct Lev {
    last: Vec<usize>,
    cur: Vec<usize>,
}

impl Default for Lev {
    fn default() -> Self {
        let lev = Self {
            last: (0..=40).collect(),
            cur: vec![0; 40],
        };
        lev
    }
}

trait Metric {
    // should be like f64 for generic metrics but we don't need that
    // for this assignment
    fn distance(&self, other: &Self) -> usize;
}

impl Metric for str {
    fn distance(&self, other: &str) -> usize {
        edit_dist(self, other)
    }
}

impl<T: ?Sized + Metric> Metric for &T {
    fn distance(&self, other: &&T) -> usize {
        <T as Metric>::distance(*self, *other)
    }
}

#[derive(Debug)]
enum VPNode<'a, T: Metric> {
    Leaf(&'a [T]),
    Branch {
        point: &'a T,
        inside: Box<VPNode<'a, T>>,
        outside: Box<VPNode<'a, T>>,
        threshold: usize,
    },
}

#[derive(Debug)]
struct VPTree<'a, T: Metric> {
    root: VPNode<'a, T>,
}

fn partition<T: Metric>(pivot: &T, s: &mut [T]) {
    // TODO: make a better partitioning scheme
    s.sort_unstable_by_key(|x| pivot.distance(x));
}

const SMALL_THRESH: usize = 10;

fn build_node<'a, T: Metric>(s: &'a mut [T]) -> VPNode<'a, T> {
    if s.len() < SMALL_THRESH {
        return VPNode::Leaf(&*s);
    }

    let random_partition = random_range(0..s.len());
    s.swap(0, random_partition);
    let mut ret = false;
    {
        let (first, rest) = s.split_first_mut().expect("Array was too small");
        partition(first, rest);
        // if all elements have the same distance, just skip the next part
        ret = first.distance(&rest[0]) == first.distance(rest.last().unwrap());
    }
    if ret {
        return VPNode::Leaf(&*s);
    }

    let (first, rest) = s.split_first_mut().unwrap();

    let middle = rest.len() / 2;
    let threshold = first.distance(&rest[middle]);
    // normally this isn't particularly useful I don't think, but we
    // have a lot of elements with the same distance.
    let partition_point = rest.partition_point(|x| x.distance(first) < threshold);
    let (inside, outside) = rest.split_at_mut(partition_point + 1);
    VPNode::Branch {
        point: first,
        inside: Box::new(build_node(inside)),
        outside: Box::new(build_node(outside)),
        threshold,
    }
}

impl<'a, T: Metric> VPTree<'a, T> {
    fn new(v: &'a mut [T]) -> Self {
        Self {
            root: build_node(v),
        }
    }

    pub fn search(&self, target: &T) -> Vec<&'a T> {
        let mut tau = usize::MAX;
        let mut results = Vec::new();
        Self::search_rec(&self.root, target, &mut results, &mut tau);
        results
    }

    fn search_rec(node: &VPNode<'a, T>, target: &T, results: &mut Vec<&'a T>, tau: &mut usize) {
        let search = Self::search_rec;

        match node {
            VPNode::Leaf(v) => Self::linear_search(*v, target, results, tau),
            VPNode::Branch {
                point,
                inside,
                outside,
                threshold,
            } => {
                let dist = target.distance(*point);
                if dist <= *tau {
                    if dist < *tau {
                        results.clear();
                        *tau = dist;
                    }
                    results.push(*point);
                }
                if dist < *threshold {
                    if dist.saturating_sub(*tau) <= *threshold {
                        search(&**inside, target, results, tau);
                    }
                    if dist + *tau >= *threshold {
                        search(&**outside, target, results, tau);
                    }
                } else {
                    if dist + *tau >= *threshold {
                        search(&**outside, target, results, tau)
                    }

                    if dist.saturating_sub(*tau) <= *threshold {
                        search(&**inside, target, results, tau)
                    }
                }
            }
        }
    }

    fn linear_search(v: &'a [T], target: &T, results: &mut Vec<&'a T>, tau: &mut usize) {
        for x in v {
            let dist = target.distance(x);
            if dist < *tau {
                *tau = dist;
                results.clear();
                results.push(x);
            } else if dist == *tau {
                results.push(x);
            }
        }
    }
}

impl Lev {
    fn init(&mut self, n: usize) {
        for j in 0..=n {
            self.last[j] = j;
        }
    }
    fn swap(&mut self) {
        mem::swap(&mut self.last, &mut self.cur);
    }
}

fn edit_dist<'a>(mut s: &'a str, mut t: &'a str) -> usize {
    static LEV: LazyLock<RwLock<Lev>> = LazyLock::new(|| RwLock::new(Lev::default()));

    let mut _lev = LEV.write().expect("poison isn't real");
    let lev = &mut *_lev;

    let mut m = s.chars().count();
    let mut n = t.chars().count();
    if n > m {
        mem::swap(&mut m, &mut n);
        mem::swap(&mut s, &mut t);
    }
    let (m, n, s, t) = (m, n, s, t);

    lev.init(n);
    // let mut last: Vec<_> = (0..=n).collect();
    // let mut cur = vec![0; n + 1];

    for (i, s_i) in s.chars().enumerate() {
        let i = i + 1;
        lev.cur[0] = i;
        for (j, t_j) in t.chars().enumerate() {
            let j = j + 1;
            let cost = if s_i == t_j { 0 } else { 1 };

            lev.cur[j] = (lev.last[j] + 1)
                .min(lev.cur[j - 1] + 1)
                .min(lev.last[j - 1] + cost);
        }
        if i != m {
            lev.swap();
        }
    }
    lev.cur[n]
}

fn edit_dist_max<'a>(mut s: &'a str, mut t: &'a str, max: usize) -> usize {
    static LEV: LazyLock<RwLock<Lev>> = LazyLock::new(|| RwLock::new(Lev::default()));

    let mut _lev = LEV.write().expect("poison isn't real");
    let lev = &mut *_lev;

    let mut m = s.chars().count();
    let mut n = t.chars().count();
    if n > m {
        mem::swap(&mut m, &mut n);
        mem::swap(&mut s, &mut t);
    }
    let (m, n, s, t) = (m, n, s, t);

    lev.init(n);
    // let mut last: Vec<_> = (0..=n).collect();
    // let mut cur = vec![0; n + 1];

    for (i, s_i) in s.chars().enumerate() {
        let i = i + 1;
        lev.cur[0] = i;
        for (j, t_j) in t.chars().enumerate() {
            let j = j + 1;
            let cost = if s_i == t_j { 0 } else { 1 };

            lev.cur[j] = (lev.last[j] + 1)
                .min(lev.cur[j - 1] + 1)
                .min(lev.last[j - 1] + cost);
        }
        if lev.cur[n] > max {
            return usize::MAX;
        }
        if i != m {
            lev.swap();
        }
    }
    lev.cur[n]
}

fn main() {
    let mut f = File::open("large/testmedordlista3.indata").expect("lol");
    // let mut f = io::stdin().lock();
    let mut s = String::new();
    f.read_to_string(&mut s).expect("lmao");
    let mut dictionary = Vec::new();
    let mut lines = s.lines();
    for line in &mut lines {
        if line == "#" {
            break;
        } else {
            dictionary.push(line);
        }
    }

    let dict2 = dictionary.clone();
    let vp = VPTree::new(&mut dictionary[..]);

    // println!("{vp:#?}");

    let mut v = vec![];

    for line in &mut lines {
        let mut min_dist = 1000;

        let start_vp = Instant::now();
        let mut vp_res = vp.search(&line);
        println!(
            "vp took {:.2} ms",
            Instant::now().duration_since(start_vp).as_secs_f64() * 1000.0
        );

        let start_linear = Instant::now();
        for word in &dict2 {
            let dist = edit_dist_max(word, line, min_dist);
            if dist < min_dist {
                min_dist = dist;
                v.clear();
                v.push(word);
            } else if dist == min_dist {
                v.push(word);
            }
        }
        println!(
            "linear took {:.2} ms",
            Instant::now().duration_since(start_linear).as_secs_f64() * 1000.0
        );

        print_results(line, line.distance(vp_res[0]), &mut vp_res);
        print_results(line, min_dist, &mut v);
    }
}

fn print_results(line: &str, min_dist: usize, v: &mut Vec<&&str>) {
    print!("{line} ({min_dist})");
    for word in v.drain(..) {
        print!(" {word}");
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::edit_dist;

    // from https://rosettacode.org/wiki/Levenshtein_distance#Rust
    fn lev_reference(word1: &str, word2: &str) -> usize {
        let w1 = word1.chars().collect::<Vec<_>>();
        let w2 = word2.chars().collect::<Vec<_>>();

        let word1_length = w1.len() + 1;
        let word2_length = w2.len() + 1;

        let mut matrix = vec![vec![0; word1_length]; word2_length];

        for i in 1..word1_length {
            matrix[0][i] = i;
        }
        for j in 1..word2_length {
            matrix[j][0] = j;
        }

        for j in 1..word2_length {
            for i in 1..word1_length {
                let x: usize = if w1[i - 1] == w2[j - 1] {
                    matrix[j - 1][i - 1]
                } else {
                    1 + std::cmp::min(
                        std::cmp::min(matrix[j][i - 1], matrix[j - 1][i]),
                        matrix[j - 1][i - 1],
                    )
                };
                matrix[j][i] = x;
            }
        }
        matrix[word2_length - 1][word1_length - 1]
    }

    macro_rules! lev {
        ($($a:ident $b:ident),*) => {
            $(
                assert_eq!(edit_dist(stringify!($a), stringify!($b)),
                           lev_reference(stringify!($a), stringify!($b)));
            )*
        };
    }

    #[test]
    fn levenshtein_edge_cases() {
        lev!(kitten kitten,
             a a
        );
    }

    #[test]
    fn normal_words_levenshtein() {
        lev!(kitten sitting,
             and sand,
             dabbbhud anbud,
             dabbbhud dabba,
             dabbbhud nabbad,
             labd labb,
             labd lagd,
             labd land,
             lol lmao
        );
    }
}
