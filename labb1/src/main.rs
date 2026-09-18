use std::io::{Write, stdin, stdout};
use std::{cmp, rc::Rc, todo};

#[derive(Debug)]
struct Tree {
    root: Option<Rc<Node>>,
    height: u32,
    history: Vec<Option<Rc<Node>>>,
}

impl Tree {
    pub fn get(&self, index: u32) -> i32 {
        if self.root.is_none() {
            return -1;
        }
        self.root
            .as_ref()
            .unwrap()
            .get(&Direction::to_direction(index, self.height))
    }

    pub fn set(&mut self, index: u32, value: i32) {
        self.history.push(self.root.clone());
        //construct the initial tree
        if self.root.is_none() {
            //if the index is zero, special case so we still have a branch node
            eprintln!("Creating initial root.");
            if index == 0 {
                self.root = Some(Rc::new(Node::one_branch(
                    Rc::new(Node::leaf(value)),
                    Direction::Left,
                )));
                self.height = 1;
                return;
            }
            eprintln!("Initial size: {}", index.highest_one().unwrap() + 1);
            //create all the branch nodes leading to our initial leaf
            let mut directions =
                Direction::to_direction_reversed(index, index.highest_one().unwrap() + 1);
            eprintln!("Directions found to be: {:?}", directions);
            self.height = directions.len() as u32;
            let mut cur = Node::leaf(value);
            for direction in directions {
                eprintln!("Creating initial node {:?}", direction);
                cur = Node::one_branch(
                    Rc::new(cur),
                    direction
                );
            }
            self.root = Some(Rc::new(cur));
            return;
        }

        //if the tree already exists, but isnt big enough, we need to extend it
        if self.height < index.highest_one().unwrap_or(0) + 1 {
            let diff = index.highest_one().unwrap_or(0) - self.height + 1;
            eprintln!("Sizing up tree... for difference of {}", diff);
            for i in 0..diff {
                eprintln!("Adding node {}", i);
                self.root = Some(Rc::new(Node::one_branch(
                    self.root.clone().unwrap(),
                    Direction::Left,
                )))
            }
            eprintln!("New height is {}", index.highest_one().unwrap_or(0) + 1);
            self.height = index.highest_one().unwrap_or(0) + 1;
        }

        //and then we crawl through the tree, saving the nodes in a vec...
        let directions = Direction::to_direction(index, self.height);
        let mut modified_nodes = vec![];
        let mut cur = self.root.clone().unwrap();
        for direction in directions {
            modified_nodes.push(cur.clone());
            if direction == Direction::Left && !cur.left.is_none() {
                cur = cur.left.clone().unwrap();
            } else if direction == Direction::Right && !cur.right.is_none() {
                cur = cur.right.clone().unwrap();
            } else {
                let mut temp = Node::leaf(0);
                temp.height = cur.height - 1;
                cur = Rc::new(temp);
            }
        }

        //and recreate those nodes in order, replacing the modified node as needed
        let mut cur = Node::leaf(value);
        let mut directions = Direction::to_direction(index, self.height);
        assert_eq!(directions.len(), modified_nodes.len());
        for (template, direction) in modified_nodes.into_iter().zip(directions) {
            if direction == Direction::Left {
                let right = template.right.clone();
                eprintln!("currently going left, adding right: {:?}", right);
                let right_max = if template.right.is_none() {
                    -1
                } else {
                    template.right.as_ref().unwrap().max
                };
                let cur_max = cur.max;
                let new_node = Node {
                    left: Some(Rc::new(cur)),
                    right,
                    max: cmp::max(cur_max, right_max),
                    height: template.height,
                };
                cur = new_node;
            } else {
                let left = template.left.clone();
                eprintln!("currently going right, adding left: {:?}", left);
                let left_max = if template.left.is_none() {
                    -1
                } else {
                    template.left.as_ref().unwrap().max
                };
                let cur_max = cur.max;
                let new_node = Node {
                    left,
                    right: Some(Rc::new(cur)),
                    max: cmp::max(left_max, cur_max),
                    height: template.height,
                };
                cur = new_node;
            }
        }

        //finally, fix the root
        self.root = Some(Rc::new(cur));
        self.height = self.root.as_ref().unwrap().height;
    }

    pub fn print_tree(&self) {
        println!("height: {}", self.height);
        fn p(a: &Option<Rc<Node>>, height: u32, mask: u32, index: u32) {
            match a.as_deref() {
                Some(Node { left: None, right: None, max: n, ..  }) => {
                    for _ in 0..height {
                        print!(" ");
                    }
                    println!("{index}: {n}");
                }
                Some(Node { left, right, .. }) => {
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

        let mask = if self.height > 0 {
            1 << self.height - 1
        } else {
            return;
        };

        p(&self.root, 0, mask, 0)
    }

    pub fn unset(&mut self) {
        todo!()
    }

    pub fn max(&self) {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    //by default, popping will give you the top level direction - aka, the root
    pub fn to_direction(index: u32, height: u32) -> Vec<Direction> {
        let mut direction_list = vec![];
        eprintln!("height: {}", height);
        for i in (1..=height).rev() {
            let cast_index = index & i * 2 - 1;
            if cast_index.highest_one() == Some(i - 1) {
                direction_list.push(Direction::Right);
            } else if cast_index.highest_one().unwrap_or(0) > i - 1 {
                panic!("Logic error in direction-conversion code.")
            } else {
                direction_list.push(Direction::Left);
            }
        }
        direction_list.into_iter().rev().collect()
    }

    //sometimes, though, you want to have the directions starting from the leaf
    pub fn to_direction_reversed(index: u32, height: u32) -> Vec<Direction> {
        let direction_list = Direction::to_direction(index, height);
        eprintln!("Reversing direction of list: {:?}.", direction_list);
        direction_list.into_iter().rev().collect()
    }
}

#[derive(Debug)]
struct Node {
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
    max: i32,
    height: u32,
}

impl Node {
    pub fn leaf(value: i32) -> Self {
        Node {
            left: None,
            right: None,
            max: value,
            height: 0,
        }
    }

    pub fn branch(left: Rc<Node>, right: Rc<Node>) -> Self {
        let max = cmp::max(left.max, right.max);
        let height = left.height + 1;
        Node {
            left: Some(left),
            right: Some(right),
            max: max,
            height: height,
        }
    }

    pub fn one_branch(node: Rc<Node>, direction: Direction) -> Self {
        let max = node.max;
        let height = node.height + 1;
        let (left, right) = if direction == Direction::Left {
            (Some(node), None)
        } else {
            (None, Some(node))
        };
        Node {
            left: left,
            right: right,
            max: max,
            height: height,
        }
    }

    pub fn get(&self, directions: &[Direction]) -> i32 {
        if self.height == 0 {
            self.max
        } else {
            let (&direction, directions) = directions
                .split_last()
                .expect("Direction list in get is inappropriate length.");
            if direction == Direction::Right && !self.right.is_none() {
                self.right.as_ref().unwrap().get(directions)
            } else if direction == Direction::Left && !self.left.is_none() {
                self.left.as_ref().unwrap().get(directions)
            } else {
                -1
            }
        }
    }

    pub fn print_node(&self, depth: u32) {
        for _ in 0..depth {
            print!("  ");
        }
        eprintln!("max: {}", self.max);
        if !self.right.is_none() {
            self.right.as_ref().unwrap().print_node(depth + 1);
        }
        if !self.left.is_none() {
            self.left.as_ref().unwrap().print_node(depth + 1);
        }
    }

    pub fn set() {
        todo!()
    }

    pub fn unset() {
        todo!()
    }

    pub fn max() {
        todo!()
    }
}

fn main() {
    let mut tree = Tree {
        root: None,
        height: 0,
        history: vec![],
    };
    loop {
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        let command: Vec<&str> = s.split(' ').collect();
        // eprintln!("commands detected: {:?}", command);
        match command[0] {
            "get" => {
                eprintln!("{}", tree.get(str::parse::<u32>(command[1]).unwrap()));
            }
            "set" => {
                tree.set(
                    str::parse::<u32>(command[1]).unwrap(),
                    str::parse::<i32>(command[2]).unwrap(),
                );
            }
            "unset" => {
                tree.unset();
            }
            "max" => {
                tree.max();
            }
            "print" => {
                tree.print_tree();
            }
            "quit" => {
                break;
            }
            _ => {
                eprintln!("wrong")
            }
        }
    }
}
