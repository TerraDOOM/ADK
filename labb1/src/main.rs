use std::{cmp, rc::Rc, todo};
use std::io::{stdin,stdout,Write};

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
        self.root.as_ref().unwrap().get(&mut Direction::to_direction(index, self.height))
    }

    pub fn set(&mut self, index: u32, value: i32) {
        self.history.push(self.root.clone());
        //construct the initial tree
        if self.root.is_none() {
            //if the index is zero, special case so we still have a branch node
            println!("Creating initial root.");
            if index == 0 {
                self.root = Some(Rc::new(Node::one_branch(Rc::new(Node::leaf(value)), Direction::Left)));
                return;
            }
            println!("Initial size: {}", index.highest_one().unwrap() + 1);
            //create all the branch nodes leading to our initial leaf
            let mut directions = Direction::to_direction_reversed(index, index.highest_one().unwrap() + 1);
            println!("Directions found to be: {:?}", directions);
            self.height = directions.len() as u32;
            let mut cur = Node::leaf(value);
            for d in 0..directions.len() {
                println!("Creating initial node {}", d);
                cur = Node::one_branch(Rc::new(cur), directions.pop().expect("Inappropriate list length in initialization of tree."));
            }
            self.root = Some(Rc::new(cur));
            return
        }

        //if the tree already exists, but isnt big enough, we need to extend it
        if self.height < index.highest_one().unwrap_or(0) + 1 {
            let diff = index.highest_one().unwrap_or(0) - self.height + 1;
            println!("Sizing up tree... for difference of {}", diff);
            for i in 0..diff {
                println!("Adding node {}", i);
                self.root = Some(Rc::new(Node::one_branch(self.root.clone().unwrap(), Direction::Left)))
            }
            println!("New height is {}", index.highest_one().unwrap_or(0) + 1);
            self.height = index.highest_one().unwrap_or(0) + 1;
        }

        //and then we crawl through the tree, saving the nodes in a vec...
        let mut directions = Direction::to_direction(index, self.height);
        let mut modified_nodes = vec!();
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
        let mut directions = Direction::to_direction_reversed(index, self.height);
        for _ in 0..modified_nodes.len() {
            let template = modified_nodes.pop().expect("modified nodes list is fucked up");
            let direction = directions.pop().expect("inappropriate direction list length when reconstructing modified nodes");
            if direction == Direction::Left {
                let right = template.right.clone();
                let right_max = if template.right.is_none() {-1} else {template.right.as_ref().unwrap().max};
                let cur_max = cur.max;
                let new_node = Node {
                    left: Some(Rc::new(cur)),
                    right: right,
                    max: cmp::max(cur_max, right_max),
                    height: template.height,
                };
                cur = new_node;
            } else {
                let left = template.left.clone();
                let left_max = if template.left.is_none() {-1} else {template.left.as_ref().unwrap().max};
                let cur_max = cur.max;
                let new_node = Node {
                    left: left,
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
        if self.root.is_none() {
            println!("Tree is empty.");
        } else {
            self.root.as_ref().unwrap().print_node(0);
        }
    }

    pub fn unset(&mut self) {
        todo!()
    }

    pub fn max(&self) {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    //by default, popping will give you the top level direction - aka, the root
    pub fn to_direction(index: u32, height: u32) -> Vec<Direction> {
        let mut direction_list = vec!();
        println!("height: {}", height);
        for i in (1..=height).rev() {
            let cast_index = index & i*2 - 1;
            if cast_index.highest_one() == Some(i-1) {
                direction_list.push(Direction::Right);
            } else if cast_index.highest_one().unwrap_or(0) > i-1 {
                panic!("Logic error in direction-conversion code.")
            } else {
                direction_list.push(Direction::Left);
            }
        };
        direction_list.into_iter().rev().collect()
    }

    //sometimes, though, you want to have the directions starting from the leaf
    pub fn to_direction_reversed(index: u32, height: u32) -> Vec<Direction> {
        let direction_list = Direction::to_direction(index, height);
        println!("Reversing direction of list: {:?}.", direction_list);
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
        let left: Option<Rc<Node>>;
        let right: Option<Rc<Node>>;
        if direction == Direction::Left {
            (left, right) = (Some(node), None);
        } else {
            (left, right) = (None, Some(node));
        }
        Node {
            left: left,
            right: right,
            max: max,
            height: height,
        }
    }
    
    pub fn get(&self, directions: &mut Vec<Direction>) -> i32 {
        if self.height == 0 {
            self.max
        } else {
            let direction = directions.pop().expect("Direction list in get is inappropriate length.");
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
        println!("max: {}", self.max);
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
        let mut s=String::new();
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        let command: Vec<&str> = s.split(' ').collect();
        println!("commands detected: {:?}", command);
        match command[0] {
            "get" => { 
                println!("{}", tree.get(str::parse::<u32>(command[1]).unwrap())); 
            }
            "set" => { 
                tree.set(str::parse::<u32>(command[1]).unwrap(), str::parse::<i32>(command[2]).unwrap()); 
            }
            "unset" => { tree.unset(); }
            "max" => { tree.max(); }
            "print" => { 
                println!("{:?}", tree);
                tree.print_tree();
            }
            "quit" => { break; }
            _ => { println!("wrong") }
        }
    }
}


