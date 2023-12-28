use std::cmp::Ordering;
use std::collections::{BTreeMap};
use priority_queue;
use std::fs;
use std::io::Read;
use priority_queue::PriorityQueue;

fn main() {
    // read file
    // TODO: read a file based on path
    let file = fs::read("files/arch-white.png");
    let byte_vec = match file {
        Ok(f) => f,
        Err(_) => panic!("Byte vector is incorrect!")
    };
    archive(byte_vec);

}
#[derive(Debug, Default, Eq, PartialEq, Hash)]
struct Node {
    value: Option<u8>,
    frequency: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn archive(bytes: Vec<u8>) {
    let map: BTreeMap<i32, u8> = build_hashmap(bytes);
    let tree: Node = build_huffman_tree(map);
    let codes: Vec<usize> = build_huffman_code(tree);
    // println!("{:?}", tree);
}

fn build_huffman_code(node: Node) -> Vec<usize> {
    let mut vec: Vec<usize> = vec![0; 10000];
    vec.fill(0);
    let top = 0;
    let vec_ref = &mut vec;
    get_code(node, top, vec_ref);
    vec

}
fn get_code(node: Node, top: usize, vec: &mut Vec<usize>) {
    let mut left_some: bool = true;
    let mut right_some: bool = true;
    let left = match node.left {
        Some(l) => {
            vec[top] = 0;
            get_code(*l, top + 1, vec);
        }
        None => {
            left_some = false;
        }
    };
    let right = match node.right {
        Some(r) => {
            vec[top] = 1;
            get_code(*r, top + 1, vec);
        }
        None => {
            right_some = false;
        }
    };
    if (left_some && right_some) {
        print!("{:?} ", node.value);
        for i in 0..top {
            print!("{:?}", vec[i])
        }
        println!();
    }
}
fn build_huffman_tree(map: BTreeMap<i32, u8>) -> Node {
    let mut nodes = map
        .into_iter()
        .map(|(k, v)| (k, Node { value: Some(v), frequency: k, left: None, right: None }))
        .collect::<PriorityQueue<i32, Node>>();
    while nodes.len() != 1 {
        let left = nodes.pop();
        let right = nodes.pop();
        let mut sum_frequency = 0;
        let left = match left {
            Some((frequency, t)) => {
                sum_frequency += frequency;
                Some(Box::new(t))
            },
            _ => { None }
        };
        let right = match right {
            Some((frequency, t)) => {
                sum_frequency += t.frequency;
                Some(Box::new(t))
            },
            _ => { None }
        };

        let node = Node {
            left,
            right,
            value: Some(0u8),
            frequency: sum_frequency,
        };
        nodes.push(sum_frequency, node);
    }
    nodes.pop().unwrap().1
}

fn build_hashmap(bytes: Vec<u8>) -> BTreeMap<i32, u8> {
    let mut map: BTreeMap<u8, i32> = BTreeMap::new();
    for byte in bytes {
        map.entry(byte).and_modify(|f| *f += 1).or_insert(1);
    }
    map.iter().map(|(k, v)| (*v, *k)).collect()
}
