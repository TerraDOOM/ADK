mod fast;

// -*- lexical-binding: t; -*-
use std::{
    cmp,
    collections::BTreeMap,
    fmt::{self, Debug, Display},
    io::{self, Write},
    iter::Zip,
    mem,
    rc::Rc,
};

enum Node {
    Leaf(i32),
    Branch {
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
        max: i32,
    },
}

impl Node {
    fn right(&self) -> Option<Rc<Node>> {
        match self {
            Node::Branch { right, .. } => right.clone(),
            _ => None,
        }
    }
    fn left(&self) -> Option<Rc<Node>> {
        match self {
            Node::Branch { left, .. } => left.clone(),
            _ => None,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Leaf(n) => f.debug_tuple("Leaf").field(n).finish(),
            Self::Branch { left, right, max } => {
                let mut s = f.debug_struct("Branch");
                s.field("max", max);
                if let Some(n) = left.as_ref() {
                    s.field("left", n);
                }
                if let Some(n) = right.as_ref() {
                    s.field("right", n);
                }
                s.finish()
            }
        }
    }
}

#[derive(Debug)]
struct Array {
    root: Option<Rc<Node>>,
    height: u32,
}

fn newarray() -> Array {
    Array {
        root: None,
        height: 0,
    }
}

#[allow(dead_code)]
fn lmao(n: &mut i32) {
    *n += 1;
    if *n > 10_000 {
        panic!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Directions {
    i: u32,
    mask: u32,
}

impl Display for Directions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.clone().collect::<Vec<_>>())
    }
}

impl Directions {
    fn new(index: u32, height: u32) -> Self {
        Directions {
            i: index,
            mask: if height == 0 { 0 } else { 1 << height - 1 },
        }
    }
}

impl Iterator for Directions {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 0 {
            None
        } else {
            let dir = self.i & self.mask;
            self.mask >>= 1;
            Some(if dir > 0 {
                Direction::Right
            } else {
                Direction::Left
            })
        }
    }
}

fn set(array: &Array, i: u32, e: i32) -> Array {
    let tgt_height = array.height.max(32 - i.leading_zeros());
    if tgt_height > array.height {
        let mut directions = Directions::new(i, tgt_height);
        assert_eq!(
            directions.next(),
            Some(Direction::Right),
            "First direction was somehow left"
        );
        let right = set_helper(None, &mut directions, e);
        let mut left = array.root.clone();
        for _ in array.height..tgt_height - 1 {
            if left.is_some() {
                left = branch(left, None);
            }
        }
        let root = branch(left, right);

        Array {
            root,
            height: tgt_height,
        }
    } else {
        let mut directions = Directions::new(i, tgt_height);
        let root = set_helper(array.root.clone(), &mut directions, e);
        Array {
            root,
            height: tgt_height,
        }
    }
}

fn branch(left: Option<Rc<Node>>, right: Option<Rc<Node>>) -> Option<Rc<Node>> {
    let max = cmp::max(get_max(&left), get_max(&right));
    Some(Rc::new(Node::Branch { left, right, max }))
}

fn leaf(n: i32) -> Option<Rc<Node>> {
    Some(Rc::new(Node::Leaf(n)))
}

fn set_helper(node: Option<Rc<Node>>, directions: &mut Directions, e: i32) -> Option<Rc<Node>> {
    let dir = directions.next();
    let mut rec = move |n| set_helper(n, directions, e);
    match (node.as_deref(), dir) {
        (Some(Node::Leaf(_)) | None, None) => leaf(e),
        (None, Some(Direction::Right)) => branch(None, rec(None)),
        (None, Some(Direction::Left)) => branch(rec(None), None),
        (Some(Node::Branch { left, right, .. }), Some(Direction::Left)) => {
            branch(rec(left.clone()), right.clone())
        }
        (Some(Node::Branch { left, right, .. }), Some(Direction::Right)) => {
            branch(left.clone(), rec(right.clone()))
        }

        (Some(Node::Leaf(_)), Some(_)) => unreachable!("Unexpected leaf node"),
        (Some(Node::Branch { .. }), None) => unreachable!("Unexpected end of direction list"),
    }
}

fn get_max(n: &Option<Rc<Node>>) -> i32 {
    match n.as_deref() {
        None => -1,
        Some(Node::Leaf(n)) => *n,
        Some(Node::Branch { max, .. }) => *max,
    }
}

fn get(array: &Array, i: u32) -> Option<i32> {
    let mut mask = if array.height > 0 {
        1 << array.height - 1
    } else {
        if i == 0 {
            return array.root.as_deref().map(|n| match n {
                Node::Leaf(i) => *i,
                _ => panic!("unexpected branch node with height 0 tree"),
            });
        } else {
            return None;
        }
    };
    if i > 2 * mask - 1 {
        return None;
    }
    let mut cur = &array.root;
    loop {
        let direction = i & mask > 0;
        match cur.as_deref() {
            Some(Node::Leaf(n)) => return Some(*n),
            Some(Node::Branch { left, right, .. }) => {
                cur = if direction { right } else { left };
            }
            None => return None,
        }
        mask >>= 1;
    }
}

fn maxininterval(array: &Array, min: u32, mut max: u32) -> Option<i32> {
    if min > max {
        return None;
    }
    if max >= (1 << array.height) {
        max = (1 << array.height) - 1;
        if min >= (1 << array.height) {
            return None;
        }
    }

    let mut dirs = (
        Directions::new(min, array.height),
        Directions::new(max, array.height),
    );

    maxininterval_rec(array.root.clone(), &mut dirs)
}

fn maxininterval_rec(
    node: Option<Rc<Node>>,
    directions: &mut (Directions, Directions),
) -> Option<i32> {
    match node.as_deref()? {
        Node::Leaf(n) => Some(*n),
        Node::Branch { left, right, .. } => {
            match (
                directions.0.next().expect("glorp"),
                directions.1.next().expect("gleeble"),
            ) {
                (Direction::Left, Direction::Right) => {
                    let right = maxright(right, &mut directions.1);
                    let left = maxleft(left, &mut directions.0);
                    cmp::max(left, right)
                }
                (Direction::Right, Direction::Right) => {
                    maxininterval_rec(right.clone(), directions)
                }
                (Direction::Left, Direction::Left) => maxininterval_rec(left.clone(), directions),
                (Direction::Right, Direction::Left) => {
                    panic!("Min told us to go right, max told us to go left???")
                }
            }
        }
    }
}

fn treemax(node: &Option<Rc<Node>>) -> Option<i32> {
    node.as_deref().map(|n| match n {
        Node::Leaf(n) => *n,
        Node::Branch { max, .. } => *max,
    })
}

fn maxleft(node: &Option<Rc<Node>>, directions: &mut Directions) -> Option<i32> {
    match node.as_deref()? {
        Node::Leaf(n) => Some(*n),
        Node::Branch { left, right, .. } => {
            match directions
                .next()
                .expect("directions have ended unexpectedly")
            {
                Direction::Right => maxleft(right, directions),
                Direction::Left => maxleft(left, directions).max(treemax(right)),
            }
        }
    }
}
fn maxright(node: &Option<Rc<Node>>, directions: &mut Directions) -> Option<i32> {
    match node.as_deref()? {
        Node::Leaf(n) => Some(*n),
        Node::Branch { left, right, .. } => {
            match directions
                .next()
                .expect("directions have ended unexpectedly")
            {
                Direction::Left => maxright(left, directions),
                Direction::Right => maxright(right, directions).max(treemax(left)),
            }
        }
    }
}

struct Set(u32, i32);

fn main() {
    let stdin = io::stdin();
    let getline = || -> io::Result<String> {
        let mut s = String::new();
        io::stdout().flush()?;
        stdin.read_line(&mut s)?;
        Ok(s)
    };

    let uint = |s: &str| s.parse::<u32>().ok();
    let mut past = Vec::new();
    let mut array = newarray();

    while let Ok(line) = { getline() } {
        if line.is_empty() {
            break;
        }
        let split = line.split_whitespace().collect::<Vec<_>>();
        match &split[..] {
            ["get", i] => {
                let Some(i) = uint(i) else { continue };
                println!("{}", get(&array, i).unwrap_or(0));
            }
            ["set", i] => {
                let Some(i) = uint(i) else { continue };
                let newarray = set(&array, i, i as i32);
                past.push(mem::replace(&mut array, newarray));
            }
            ["set", i, e] => {
                let Some(i) = uint(i) else { continue };
                let Some(e) = uint(e) else { continue };
                dbg!((i, e));
                let newarray = set(&array, i, e as i32);
                past.push(mem::replace(&mut array, newarray));
            }
            ["unset"] => {
                let mut last = past.pop().unwrap_or(newarray());
                mem::swap(&mut last, &mut array);
                mem::forget(last);
            }
            ["maxininterval" | "max", lo, hi] => {
                let Some(lo) = uint(lo) else {
                    continue;
                };
                let Some(hi) = uint(hi) else { continue };
                println!("{}", maxininterval(&array, lo, hi).unwrap_or(0));
            }
            ["print", ..] => {
                print_tree(&array);
            }
            ["break"] => break,
            _ => continue,
        }
    }
}

fn print_tree(array: &Array) {
    println!("height: {}", array.height);
    fn p(a: &Option<Rc<Node>>, height: u32, mask: u32, index: u32) {
        match a.as_deref() {
            Some(Node::Leaf(n)) => {
                for _ in 0..height {
                    print!(" ");
                }
                println!("{index}: {n}");
            }
            Some(Node::Branch { left, right, .. }) => {
                for _ in 0..height {
                    print!(" ");
                }
                println!(
                    "(branch: {}-{})",
                    index,
                    index + (mask * 2).saturating_sub(1)
                );
                p(left, height + 2, mask >> 1, index);
                p(right, height + 2, mask >> 1, index + mask);
            }
            None => return,
        }
    }

    let mask = if array.height > 0 {
        1 << array.height - 1
    } else {
        return;
    };

    p(&array.root, 0, mask, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[derive(Default)]
    struct ArrayButLmao {
        sets: Vec<Set>,
        map: BTreeMap<u32, i32>,
    }

    impl ArrayButLmao {
        fn new() -> Self {
            Default::default()
        }
        fn set(&mut self, i: u32, e: i32) {
            self.sets.push(Set(i, e));
            self.map.insert(i, e);
        }
        fn get(&self, i: u32) -> Option<i32> {
            self.map.get(&i).copied()
        }
        fn unset(&mut self) {
            if let Some(Set(i, _)) = self.sets.pop() {
                self.map.remove(&i);
                for &Set(j, e) in self.sets.iter().rev() {
                    if i == j {
                        self.map.insert(i, e);
                        break;
                    }
                }
            }
        }
        fn maxininterval(&mut self, lo: u32, hi: u32) -> Option<i32> {
            self.map.range(lo..=hi).map(|(_, &e)| e).max()
        }
    }

    #[derive(Copy, Clone)]
    enum Instruction {
        Set(u32, i32),
        Get(u32),
        Unset,
        Max(u32, u32),
    }
    impl Debug for Instruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Set(arg0, arg1) => write!(f, "set {arg0} {arg1}"),
                Self::Get(arg0) => write!(f, "get {arg0}"),
                Self::Unset => write!(f, "unset"),
                Self::Max(arg0, arg1) => write!(f, "max {arg0} {arg1}"),
            }
        }
    }

    macro_rules! c {
        (set $i:literal $e:literal) => {
            Instruction::Set($i, $e)
        };
        (get $i:literal) => {
            Instruction::Get($i)
        };
        (unset) => {
            Instruction::Unset
        };
        (max $lo:literal $hi:literal) => {
            Instruction::Max($lo, $hi)
        };
        (maxininterval $lo:literal $hi:literal) => {
            Instruction::Max($lo, $hi)
        };
    }

    #[inline(never)]
    fn compare_reference_to_actual(instructions: &[Instruction]) {
        let mut past = Vec::new();
        let mut array = newarray();
        let mut good_array = ArrayButLmao::new();

        for (index, &instr) in instructions.iter().enumerate() {
            match instr {
                Instruction::Set(i, e) => {
                    let newarray = set(&array, i, e as i32);
                    past.push(mem::replace(&mut array, newarray));
                    good_array.set(i, e as i32);
                }
                Instruction::Get(i) => {
                    if get(&array, i) != good_array.get(i) {
                        println!(
                            "panicked at index {index}, current instruction: {:?}",
                            instructions[index]
                        );
                        println!("instructions: {:#?}", &instructions[0..=index]);
                        panic!()
                    }
                }
                Instruction::Unset => {
                    let mut last = past.pop().unwrap_or(newarray());
                    mem::swap(&mut last, &mut array);
                    mem::forget(last);
                    good_array.unset();
                }
                Instruction::Max(lo, hi) => {
                    let got = maxininterval(&array, lo, hi);
                    let expected = good_array.maxininterval(lo, hi);
                    if got != expected {
                        print_tree(&array);
                        println!(
                            "panicked at index {index}, current instruction: {:?}",
                            instructions[index]
                        );
                        println!("Got: {got:?}\nExpected: {expected:?}");
                        println!("instructions: {:#?}", &instructions[0..=index]);
                        panic!()
                    }
                }
            }
        }
    }

    #[inline(never)]
    fn generate_random_test(rng: &mut StdRng, n: usize) -> Vec<Instruction> {
        let mut v = vec![];
        let mut current_indices = vec![];
        for _ in 0..n {
            match rng.random_range(0..7) {
                0 | 4 | 5 => {
                    let a = rng.random_range(0..1024);
                    let b = rng.random_range(0..1_000_000);

                    v.push(Instruction::Set(a, b));
                    current_indices.push(a);
                }
                1 => {
                    v.push(Instruction::Unset);
                    current_indices.pop();
                }
                2 => v.push(Instruction::Get(rng.random_range(0..1024))),
                3 => {
                    let lo = rng.random_range(0..1023);
                    let hi = rng.random_range(lo..1024);
                    v.push(Instruction::Max(lo, hi));
                }
                6 if current_indices.len() != 0 => v.push(Instruction::Get(
                    current_indices[rng.random_range(0..current_indices.len())],
                )),
                _ => continue,
            }
        }
        v
    }

    #[test]
    fn normal_maxininterval() {
        let seq = vec![
            c!(set 1 1),
            c!(set 2 2),
            c!(set 3 3),
            c!(set 4 4),
            c!(set 5 5),
            c!(maxininterval 1 4),
            c!(maxininterval 6 10),
            c!(unset),
            c!(unset),
            c!(maxininterval 1 4),
        ];
        compare_reference_to_actual(&seq[..]);
    }

    #[test]
    fn normal_first() {
        let seq = vec![
            c!(set 3 17),
            c!(set 3 4711),
            c!(get 3),
            c!(set 2 20),
            c!(maxininterval 1 3),
            c!(unset),
            c!(set 3 1000),
            c!(unset),
            c!(get 3),
            c!(unset),
            c!(get 3),
        ];
        compare_reference_to_actual(&seq[..]);
    }

    #[test]
    fn small_random() {
        const N: u32 = 100;

        let mut rng = StdRng::seed_from_u64(42069);
        let mut v = vec![];

        for _ in 0..N {
            match rng.random_range(0..6) {
                0 | 4 | 5 => v.push(Instruction::Set(
                    rng.random_range(0..1024),
                    rng.random_range(0..1_000_000),
                )),
                1 => v.push(Instruction::Unset),
                2 => v.push(Instruction::Get(rng.random_range(0..1024))),
                3 => {
                    let lo = rng.random_range(0..1023);
                    let hi = rng.random_range(lo..1024);
                    v.push(Instruction::Max(lo, hi));
                }
                _ => panic!(),
            }
        }

        compare_reference_to_actual(&v[..]);
    }

    #[test]
    fn bigger_random() {
        const N: usize = 100_000;
        const M: usize = 10;
        let mut rng = StdRng::seed_from_u64(42069);

        for m in 0..M {
            println!("running test {m}");

            let v = generate_random_test(&mut rng, N);

            compare_reference_to_actual(&v[..]);
        }
    }
}
