use std::cmp::{Ordering};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Node {
    pub value: Value,
    pub left: Branch,
    pub right: Branch,
}

impl Node {
    fn new(value: Value) -> Self {
        Self { value, left: Branch(None), right: Branch(None) }
    }
}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Value {
    pub word: String,
    pub frequency: usize,
}

pub fn values_from_words(words: Vec<String>, tree: &mut Tree) {
    for word in words {
        tree.insert(word);
    }
}

#[derive(Debug, Clone)]
pub struct Branch(Option<Box<Node>>);

#[derive(Debug)]
pub struct Tree {
    pub root: Branch,
    len: usize,
}

impl Tree {
    pub fn new() -> Self {
        Self { root: Branch::new(), len: 0 }
    }
    pub fn insert(&mut self, value: String) {
        self.root.insert(value);
        self.len += 1;
    }
    pub fn find(&self, word: String) -> Option<Value> {
        self.root.find(word)
    }

    pub fn get_list_frequency(&self, file: &mut File) {
        self.root.get_list_frequency(file, self.len);
    }

    pub fn get_list_filtered(&self, file: &mut File, len: usize) {
        self.root.get_list_filtered(file, len);
    }

    pub fn get_list_alphabetical(&self, file: &mut File) {
        self.root.get_list_alphabetical(file)
    }
}

impl Branch {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn insert(&mut self, value: String) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node::new(Value{ word: value, frequency: 1} ))),
            Some(n) => match value.cmp(&n.value.word) {
                Ordering::Less => n.left.insert(value),
                Ordering::Equal => n.value.frequency += 1,
                Ordering::Greater => n.right.insert(value),
            },
        }
    }
    pub fn find(&self, word: String) -> Option<Value> {
        match &self.0 {
            None => None,
            Some(n) => match word.cmp(&n.value.word) {
                Ordering::Less => n.left.find(word),
                Ordering::Equal => Some(n.value.clone()),
                Ordering::Greater => n.right.find(word),
            },
        }
    }

    pub fn get_list_frequency(&self, file: &mut File, size: usize) {
        let mut vec = Vec::with_capacity(size);
        self.traverse(&mut vec);
        vec.sort_by(|x, y| x.frequency.cmp(&y.frequency));
        for val in vec {
            writeln!(file, "{} {}", val.word, val.frequency)
                .expect("TODO: panic message");
        }
    }
    fn traverse(&self, vec: &mut Vec<Value>) {
        if self.0.is_none() {
            return;
        }
        match &self.0 {
            None => {}
            Some(n) => {
                n.left.traverse(vec);
                vec.push(self.0.clone().unwrap().value);
                n.right.traverse(vec);
            }
        }
    }
    pub fn get_list_alphabetical(&self, file: &mut File) {
        if self.0.is_none() {
            return;
        }
        match &self.0 {
            None => {}
            Some(n) => {
                n.left.get_list_alphabetical(file);
                let val = self.0.clone().unwrap();
                writeln!(file, "{} {}", val.value.word, val.value.frequency)
                    .expect("TODO: panic message");
                n.right.get_list_alphabetical(file);
            }
        }
    }
    pub fn get_list_filtered(&self, file: &mut File, len: usize) {
        if self.0.is_none() {
            return;
        }
        match &self.0 {
            None => {}
            Some(n) => {
                n.left.get_list_filtered(file, len);
                let val = self.0.clone().unwrap();
                if val.value.word.chars().count() == len {
                    writeln!(file, "{} {}", val.value.word, val.value.frequency)
                        .expect("TODO: panic message");
                }
                n.right.get_list_filtered(file, len);
            }
        }
    }
}