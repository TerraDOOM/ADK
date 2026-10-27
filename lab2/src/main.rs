use std::{
    collections::{BTreeSet, BinaryHeap},
    fs::File,
    hint::unreachable_unchecked,
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

impl Lev {
    fn init(&mut self, n: usize) {
        for i in 0..n {
            self.last[i] = i;
        }
    }

    fn swap(&mut self) {
        mem::swap(&mut self.last, &mut self.cur);
    }
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

#[derive(Default)]
struct Trie<'a> {
    children: Vec<(char, Trie<'a>)>,
    word: Option<&'a str>,
}

fn print_trie(node: &Trie, indent: usize) {
    if let Some(terminal) = node.word {
        for i in 0..indent {
            print!("  ");
        }
        println!("{terminal}");
    }
    for (pre, next) in &node.children {
        for i in 0..indent {
            print!("  ");
        }
        println!("{pre}");
        print_trie(next, indent + 1);
    }
}

impl<'a> Trie<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn add_inner(&mut self, s: &'a str, word: &'a str) {
        let mut chars = s.chars();
        let Some(first) = chars.next() else {
            self.word = Some(word);
            return;
        };
        let rest = chars.as_str();

        for (pre, child) in &mut self.children {
            if *pre == first {
                return child.add_inner(rest, word);
            }
        }
        self.children.push((first, Trie::new()));
        self.children.last_mut().unwrap().1.add_inner(rest, word);
    }

    fn add(&mut self, s: &'a str) {
        self.add_inner(s, s);
    }
}

struct LevTraversal<'a> {
    trie: &'a Trie<'a>,
    sizes: BTreeSet<usize>,
    min_dist: u8,
    min_words: Vec<&'a str>,
}

impl<'a> LevTraversal<'a> {
    fn find(&mut self, word: &'a str) -> (usize, &[&'a str]) {
        self.min_words.clear();
        let init: Vec<_> = (0..=word.len() as u8).collect();

        let min_max_size = self
            .sizes
            .iter()
            .filter(|x| **x >= word.len())
            .map(|x| *x)
            .min()
            .unwrap_or(word.len());
        for i in 0..min_max_size {
            self.min_dist = i as u8;
            for (pre, next) in &self.trie.children {
                self.search(next, word, *pre, &init[..]);
            }
            // println!("swept {i} for {word}, found: {:?}", self.min_words);
            if !self.min_words.is_empty() {
                break;
            }
        }

        (self.min_dist as usize, &self.min_words[..])
    }

    fn search(&mut self, node: &Trie<'a>, word: &'a str, letter: char, last: &[u8]) {
        let mut state = vec![0; last.len()];

        state[0] = last[0] + 1;
        let mut min = u8::MAX;
        for i in 1..last.len() {
            let cost = if word.chars().nth(i - 1) == Some(letter) {
                0
            } else {
                1
            };

            state[i] = (last[i] + 1).min(state[i - 1] + 1).min(last[i - 1] + cost);
            min = min.min(state[i]);
        }
        if node.word.is_some() && state[state.len() - 1] <= self.min_dist {
            let Some(terminal) = node.word else { panic!() };
            if state[state.len() - 1] < self.min_dist {
                self.min_dist = state[state.len() - 1];
                self.min_words.clear();
            }
            self.min_words.push(terminal);
        }

        // println!("at distance {min} on character {letter}");

        if min <= self.min_dist {
            for (pre, next) in &node.children {
                self.search(next, word, *pre, &state[..]);
            }
        }
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
        for (j, t_j) in t
            .chars()
            .enumerate()
        {
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

fn edit_dist_max<'a>(mut s: &'a str, mut t: &'a str, k: usize) -> usize {
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
        let mut min = i;
        for (j, t_j) in t
            .chars()
            .enumerate()
            .skip(i.saturating_sub(k).saturating_sub(1).clamp(0, k))
            .take(2 * k + 1)
        {
            let j = j + 1;
            let cost = if s_i == t_j { 0 } else { 1 };

            lev.cur[j] = (lev.last[j] + 1)
                .min(lev.cur[j - 1] + 1)
                .min(lev.last[j - 1] + cost);
            min = min.min(lev.cur[j]);
        }
        dbg!(&lev.cur[0..n]);
        if min > k {
            return min;
        }
        if i != m {
            mem::swap(&mut lev.cur, &mut lev.last);
        }
    }
    lev.cur[n]
}

fn main() {
    let start_linear = Instant::now();

    // let mut f = File::open("large/testmedordlista3.indata").expect("lol");
    let mut f = io::stdin().lock();
    let mut s = String::new();
    f.read_to_string(&mut s).expect("lmao");
    let mut dictionary = Vec::new();
    let mut lines = s.lines();
    let mut sizes = BTreeSet::new();
    for line in &mut lines {
        if line == "#" {
            break;
        } else {
            dictionary.push(line);
            sizes.insert(line.len());
        }
    }

    let start = Instant::now();
    // let mut trie = Trie::new();
    // for word in &dictionary {
    //     trie.add(*word);
    // }
    // // print_trie(&trie, 0);
    // let mut lev = LevTraversal {
    //     trie: &trie,
    //     sizes,
    //     min_dist: 0,
    //     min_words: Vec::new(),
    // };
    // let finish = Instant::now().duration_since(start).as_secs_f64() * 1000.0;
    //eprintln!("building trie took {:.2} ms", finish);

    let mut v = vec![];

    for line in &mut lines {
        // let start_vp = Instant::now();
        // let (min_dist_trie, res) = lev.find(line);
        // let finish_search = Instant::now().duration_since(start_vp).as_secs_f64() * 1000.0;

        // print_results(line, min_dist_trie, res);
        // eprintln!("trie took {:.2} ms", finish_search);

        let mut min_dist = 40;
        for &word in &dictionary {
            let dist = dbg!(edit_dist_max(dbg!(word), line, min_dist));
            if dist < min_dist {
                min_dist = dist;
                v.clear();
                v.push(word);
            } else if dist == min_dist {
                v.push(word);
            }
        }

        print_results(line, min_dist, &v[..]);
    }
    // println!(
    //     "linear took {:.2} ms",
    //     Instant::now().duration_since(start_linear).as_secs_f64() * 1000.0
    // );
}

fn print_results(line: &str, min_dist: usize, v: &[&str]) {
    print!("{line} ({min_dist})");
    for word in v.iter() {
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
