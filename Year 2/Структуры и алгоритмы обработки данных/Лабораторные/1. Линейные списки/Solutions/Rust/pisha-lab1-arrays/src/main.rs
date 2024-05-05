use std::ptr;
use rand::{random, Rng};

#[derive(Debug)]

pub struct StackList {
    head: StackLink,
}

type StackLink = Option<Box<StackNode>>;

#[derive(PartialEq, Debug)]
struct StackNode {
    elem: i32,
    next: StackLink,
}
impl StackList {
    pub fn new() -> Self {
        StackList { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        // Box - smart pointer, points to data on a heap
        let new_node = Box::new(StackNode {
            elem,
            // As we store values as Options,
            // we need to take out the value out of the Option
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    fn print(&self) -> String {
        let mut str: String = " ".to_string();
        let mut node = &self.head;
        loop {
            if node.is_some() {
                str += &node.as_ref().unwrap().elem.to_string();
                str += " ";
                node = &node.as_ref().unwrap().next;
            } else {
                break
            }
        }
        str
    }
    pub fn empty(&mut self) -> bool {
        self.head == None
    }
}

impl Drop for StackList {
    // manual drop implementation because linked lists are
    // bad in terms of ownership
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
#[derive(Debug)]
pub struct QueueList {
    head: QueueLink,
    tail: *mut QueueNode,
}
type QueueLink = *mut QueueNode;
struct QueueNode {
    elem: i32,
    next: QueueLink,
}
impl QueueList {
    pub fn new() -> Self {
        QueueList { head: ptr::null_mut(), tail: ptr::null_mut() }
    }
    pub fn enqueue(&mut self, elem: i32) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(QueueNode {
                elem,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }
    pub fn dequeue(&mut self) -> Option<i32> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)
            }
        }
    }
    pub fn print(&mut self) -> String {
        let mut str: String = String::default();
        let mut head = self.head;
        unsafe {
            while (*head).next.as_ref().is_some() {
                str += &(*head).elem.clone().to_string();
                str += " ";
                head = (*head).next;
            }
        }
        str
    }
    fn empty(&self) -> bool {
        self.head.is_null()
    }
}

fn main() {
    let mut stack_1 = StackList::new();
    let size_1: i32 = rand::thread_rng().gen_range(2..20);
    for _ in 1..size_1 {
        stack_1.push((random::<u32>() % 100 + 1) as i32);
    }
    println!("Stack 1: {}", stack_1.print());
    let mut stack_2 = StackList::new();
    let size_2: i32 = rand::thread_rng().gen_range(2..20);
    for _ in 1..size_2 {
        stack_2.push((random::<u32>() % 100 + 1) as i32);
    }
    println!("Stack 2: {}", stack_2.print());
    let mut queue = QueueList::new();
    let mut min: i32 = i32::MAX;
    while !stack_2.empty() {
        let temp = stack_2.pop().unwrap();
        if temp < min {
            min = temp
        }
    }
    while !stack_1.empty() {
        let i = stack_1.pop().unwrap();
        if min != 0 && i % min == 0 {
            queue.enqueue(i);
        }
    }
    println!("Min: {}", min);
    let print3 = queue.print();
    println!("Queue: {}", print3);
}