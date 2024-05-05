use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Default, Eq, PartialEq, Hash)]
pub struct Node {
    pub value: Option<u8>,
    pub frequency: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.frequency == other.frequency {
            return self.value.cmp(&other.value);
        };
        self.frequency.cmp(&other.frequency)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// struct FrequencyMap(BTreeMap<u8, i32>);
// struct HuffmanTree(Box<Node>);
// struct CodeMap(HashMap<u8, String>);
pub fn build_huffman_tree(map: &mut BTreeMap<u8, i32>) -> Box<Node> {
    let mut nodes = map
        .into_iter()
        .map(|(k, v)| Some(Box::new(Node { value: Some(*k), frequency: *v, left: None, right: None })))
        .collect::<Vec<Option<Box<Node>>>>();
    nodes.sort();
    // dbg!(&nodes);
    while nodes.len() != 1 {
        let left = nodes.remove(0);
        let right = nodes.remove(0);

        let mut sum_frequency = 0;
        let left = match left {
            Some(frequency) => {
                sum_frequency += frequency.frequency;
                Some(frequency)
            },
            _ => { None }
        };
        let right = match right {
            Some(frequency) => {
                sum_frequency += frequency.frequency;
                Some(frequency)
            },
            _ => { None }
        };

        let node = Node {
            left,
            right,
            value: None,
            frequency: sum_frequency,
        };
        nodes.push(Some(Box::new(node)));
        nodes.sort();
        // dbg!(&nodes);
    }
    Box::new(*nodes.pop().unwrap().unwrap())
}
pub fn build_huffman_code(node: Box<Node>) -> HashMap<u8, String> {
    let mut hash: HashMap<u8, String> = HashMap::new();
    // let hash_ref = &mut hash;
    get_code(Some(node), "".to_string(), &mut hash);
    hash

}
fn get_code(node: Option<Box<Node>>, string: String, hash: &mut HashMap<u8, String>) {
    if let Some(node) = node {
        if let Some(v) = node.value {
            hash.insert(v, string.clone());
        } else {
            get_code(node.left, string.clone() + "0", hash);
            get_code(node.right, string.clone() + "1", hash);
        }
    } else {
        return;
    }
}

#[cfg(test)]
mod test {
    use std::collections::{BTreeMap, HashMap};
    use crate::structs::{build_huffman_code, build_huffman_tree, Node};

    #[test]
    fn test_priority_sort() {
        let mut node_vec = vec![];
        node_vec.push(Some(Box::new(Node {frequency: 10, right: None, left: None, value: Some(10) })));
        node_vec.push(Some(Box::new(Node {frequency: 20, right: None, left: None, value: Some(20) })));
        node_vec.push(Some(Box::new(Node {frequency: 10, right: None, left: None, value: Some(50) })));
        node_vec.sort();
        let correct_sort = vec![
                                Some(Box::new(Node {frequency: 10, right: None, left: None, value: Some(10) })),
                                Some(Box::new(Node {frequency: 10, right: None, left: None, value: Some(50) })),
                                Some(Box::new(Node {frequency: 20, right: None, left: None, value: Some(20) }))
        ];
        assert_eq!(node_vec, correct_sort);

    }

    #[test]
    fn test_huffman_tree() {
        let mut correct_map: BTreeMap<u8, i32> = BTreeMap::new();
        correct_map.insert('A' as u8, 5);
        correct_map.insert('B' as u8, 9);
        correct_map.insert('C' as u8, 12);
        correct_map.insert('D' as u8, 13);
        correct_map.insert('E' as u8, 16);
        correct_map.insert('F' as u8, 45);
        let test_tree = build_huffman_tree(&mut correct_map);
        println!("tree: {test_tree:?}");
        assert_eq!(test_tree.frequency, 100);
        assert_eq!(test_tree.left.as_ref().unwrap().value, Some('F' as u8));
    }
    #[test]
    fn test_huffman_codes() {
        let mut correct_map: BTreeMap<u8, i32> = BTreeMap::new();
        correct_map.insert('A' as u8, 5);
        correct_map.insert('B' as u8, 9);
        correct_map.insert('C' as u8, 12);
        correct_map.insert('D' as u8, 13);
        correct_map.insert('E' as u8, 16);
        correct_map.insert('F' as u8, 45);

        let mut correct_codes: HashMap<u8, String> = HashMap::new();
        correct_codes.insert('A' as u8, "1100".to_owned());
        correct_codes.insert('C' as u8, "100".to_owned());
        correct_codes.insert('B' as u8, "1101".to_owned());
        correct_codes.insert('D' as u8, "101".to_owned());
        correct_codes.insert('E' as u8, "111".to_owned());
        correct_codes.insert('F' as u8, "0".to_owned());

        let tree = build_huffman_tree(&mut correct_map);
        let test_codes = build_huffman_code(tree);
        assert_eq!(correct_codes, test_codes);
    }
}