// -*- lexical-binding: t; -*-
use std::{
    cmp,
    collections::BTreeMap,
    fmt::{self, Debug},
    io::{self, Write},
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

fn set(array: &Array, i: u32, e: i32) -> Array {
    let tgt_height_mask = if i > 0 {
        1 << 31 - i.leading_zeros()
    } else {
        0
    };
    let current_height_mask = match array.height {
        0 => 0,
        1 => 1,
        h => 1 << h - 1,
    };
    let mask = cmp::max(tgt_height_mask, current_height_mask);
    if tgt_height_mask > current_height_mask {
        if cfg!(debug_assertions) && !cfg!(test) {
            eprintln!("adding more layers to tree");
        }
        let mut cur = array.root.clone();
        let max = get_max(&array.root);
        let mut cur_height = current_height_mask;
        while cur_height << 1 < tgt_height_mask {
            cur = Some(Rc::new(Node::Branch {
                left: cur,
                right: None,
                max,
            }));
            if cur_height == 0 {
                cur_height = 1;
            } else {
                cur_height <<= 1;
            }
        }
        let root = branch(cur, set_helper(None, i, e, mask >> 1));
        Array {
            root,
            height: 32 - tgt_height_mask.leading_zeros(),
        }
    } else {
        let root = set_helper(array.root.clone(), i, e, mask);
        Array {
            root,
            height: cmp::max(array.height, 32 - i.leading_zeros()),
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

fn set_helper(node: Option<Rc<Node>>, i: u32, e: i32, mask: u32) -> Option<Rc<Node>> {
    if cfg!(debug_assertions) && !cfg!(test) {
        eprintln!("set_helper: {node:?}, {i}, {e}, {mask:0b}");
    }
    let rec = move |n| set_helper(n, i, e, mask >> 1);
    if mask == 0 {
        return leaf(e);
    }
    let direction = i & mask > 0;
    match node.as_deref() {
        None => {
            let new_node = rec(None);
            let (left, right) = if direction {
                (None, new_node)
            } else {
                (new_node, None)
            };
            return branch(left, right);
        }
        Some(Node::Branch { left, right, .. }) => {
            let (left, right) = if direction {
                (left.clone(), rec(right.clone()))
            } else {
                (rec(left.clone()), right.clone())
            };
            return branch(left, right);
        }
        Some(Node::Leaf(_)) => unreachable!("Unexpected leaf node"),
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
        } else { return None }
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

fn maxininterval(array: &Array, min: u32, max: u32) -> Option<i32> {
    let mask = match array.height {
        0 => 0,
        1 => 1,
        h => 1 << h - 1,
    };
    if min > max || array.root.is_none() || min > 2 * mask - 1 {
        return None;
    }

    match (max >= 2 * mask - 1, min == 0) {
        (true, true) => treemax(&array.root),
        (false, true) => maxright(&array.root, max, mask),
        (true, false) => maxleft(&array.root, min, mask),
        (false, false) => maxininterval_rec(&array.root, min, max, mask),
    }
}

fn maxininterval_rec(node: &Option<Rc<Node>>, lo: u32, hi: u32, mask: u32) -> Option<i32> {
    if cfg!(debug_assertions) && !cfg!(test) {
        eprintln!("lo: {lo}, hi: {hi}, treemax({:?})", treemax(node));
    }
    match node.as_deref()? {
        Node::Leaf(n) => Some(*n),
        Node::Branch { left, right, .. } => {
            let hi_direction = hi & mask > 0;
            let lo_direction = lo & mask > 0;
            match (hi_direction, lo_direction) {
                (true, false) => {
                    let right = maxright(right, hi, mask >> 1);
                    let left = maxleft(left, lo, mask >> 1);
                    cmp::max(left, right)
                }
                (true, true) => maxininterval_rec(right, lo, hi, mask >> 1),
                (false, false) => maxininterval_rec(left, lo, hi, mask >> 1),
                (false, true) => panic!("wtf"),
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

fn maxleft(node: &Option<Rc<Node>>, lo: u32, mask: u32) -> Option<i32> {
    if cfg!(debug_assertions) && !cfg!(test) {
        eprintln!("(maxleft) lo: {lo:b}, mask: {mask:b}");
    }
    match node.as_deref()? {
        Node::Leaf(n) => Some(*n),
        Node::Branch { left, right, .. } => {
            let direction = lo & mask > 0;
            if direction {
                maxleft(right, lo, mask >> 1)
            } else {
                let cont = maxleft(left, lo, mask >> 1);
                cmp::max(cont, treemax(right))
            }
        }
    }
}
fn maxright(node: &Option<Rc<Node>>, hi: u32, mask: u32) -> Option<i32> {
    if cfg!(debug_assertions) && !cfg!(test) {
        eprintln!("(maxright) hi: {hi:b}, mask: {mask:b}");
    }
    match node.as_deref()? {
        Node::Leaf(n) => Some(*n),
        Node::Branch { left, right, .. } => {
            let direction = hi & mask > 0;
            if direction {
                let cont = maxright(right, hi, mask >> 1);
                cmp::max(cont, treemax(left))
            } else {
                maxright(left, hi, mask >> 1)
            }
        }
    }
}

struct Set(u32, i32);
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

fn main() -> io::Result<()> {
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
    let mut good_array = ArrayButLmao::new();

    while let Ok(line) = {
        // eprint!("> ");
        getline()
    } {
        if line.is_empty() {
            break;
        }
        // println!("> {line}");
        let split = line.split_whitespace().collect::<Vec<_>>();
        match &split[..] {
            ["get", i] => {
                let Some(i) = uint(i) else { continue };
                eprintln!("reference answer: {}", good_array.get(i).unwrap_or(0));
                println!("{}", get(&array, i).unwrap_or(0));
            }
            ["set", i] => {
                let Some(i) = uint(i) else { continue };
                let newarray = set(&array, i, i as i32);
                past.push(mem::replace(&mut array, newarray));
                good_array.set(i, i as i32);
            }
            ["set", i, e] => {
                let Some(i) = uint(i) else { continue };
                let Some(e) = uint(e) else { continue };
                dbg!((i, e));
                let newarray = set(&array, i, e as i32);
                past.push(mem::replace(&mut array, newarray));
                good_array.set(i, e as i32);
            }
            ["unset"] => {
                let last = past.pop().unwrap_or(newarray());
                array = last;
                good_array.unset();
            }
            ["maxininterval" | "max", lo, hi] => {
                let Some(lo) = uint(lo) else {
                    continue;
                };
                let Some(hi) = uint(hi) else { continue };
                println!("{}", maxininterval(&array, lo, hi).unwrap_or(0));
                eprintln!(
                    "reference answer: {}",
                    good_array.maxininterval(lo, hi).unwrap_or(0)
                );
            }
            ["print", ..] => {
                print_tree(&array);
            }
            ["break"] => break,
            _ => continue,
        }
    }

    Ok(())
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
                    let last = past.pop().unwrap_or(newarray());
                    array = last;
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
        let mut rng = StdRng::seed_from_u64(420);

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
        const N: u32 = 1000;
        const M: u32 = 1000;
        let mut rng = StdRng::seed_from_u64(420);

        for m in 0..M {
            let mut v = vec![];

            println!("running test {m}");

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

            if m == 936 {
                for i in &v {
                    println!("{:?}", i);
                }
            }
            compare_reference_to_actual(&v[..]);
        }
    }
}
