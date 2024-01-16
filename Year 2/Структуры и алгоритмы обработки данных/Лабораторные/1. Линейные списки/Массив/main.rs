#![feature(new_uninit)]

use std::mem::ManuallyDrop;
use std::ptr;
use rand::{random, Rng};

#[derive(Debug)]
struct Stack {
    top: i32,
    data: Box<[Option<i32>]>,
}
impl Stack {
    fn new(capacity: u32) -> Self {
        let top = -1;
        // creating vector because you can't create fixed runtime-size array
        // filling with Nones because otherwise .into_boxed_slice() will trim the unused capacity
        let mut data: Vec<Option<i32>> = vec![None; capacity as usize];
        // Box is a pointer to data on stack so boxed slice is basically an array
        let data = data.into_boxed_slice();
        Stack {
            top,
            data,
        }
    }
    fn push(&mut self, value: i32) {
        self.top += 1;
        self.data[self.top as usize] = Some(value);
    }
    fn pop(&mut self) -> Result<i32, String> {
        if self.top != 0 {
            let ret: i32 = self.data[self.top as usize].unwrap();
            self.top -= 1;
            Ok(ret)
        }
        else {
            return Err("stack is empty".to_string());
        }
    }
    fn null_stack(&mut self) {
        self.top = -1;
    }
    fn empty(&self) -> bool {
        self.top <= 0
    }
    fn print(&self) -> String {
        let mut str: String = String::default();
        let mut top = self.top;
        while top > -1 {
            str += &self.data[top as usize].unwrap().to_string();
            str += " ";
            top -= 1;
        }
        str
    }
}
struct Queue {
    data: Box<[Option<i32>]>,
    tail: usize,
    head: usize,
    size: usize,
}
impl Queue {
    fn new(cap: usize) -> Self {
        let boxed = vec![None; cap].into_boxed_slice();
        Queue {
            data: boxed,
            tail: 0,
            head: 0,
            size: cap,
        }
    }
    fn enqueue(&mut self, value: i32) {
        if self.tail != self.size {
            self.data[self.tail] = Some(value);
            self.tail += 1;
        }
        else {
            println!("queue is full");
        }
    }
    fn dequeue(&mut self) -> Result<i32, &str> {
        let ret = self.data[self.tail - 1];
        if self.tail != self.head {
            for i in 0..self.tail - 1 {
                self.data[i] = self.data[i + 1];
            }
            // decrement rear
            self.tail -= 1;
            Ok(ret.unwrap())
        }
        else {
            Err("queue is empty")
        }
    }
    fn print(&self) -> String {
        let mut str = String::default();
        for i in self.head..self.tail {
            match &self.data[i] {
                Some(i) => {
                    i.to_string();
                    str += &i.to_string();
                }
                None => {
                }
            }
            str += &" ";
        }
        str
    }
    fn null_queue(&mut self) {
        while self.tail != self.head {
            self.data[self.tail] = None;
            self.tail -= 1;
        }
    }
}

fn main() {
    let mut queue_test: Queue = Queue::new(5);
    queue_test.enqueue(1);
    queue_test.enqueue(2);
    queue_test.enqueue(3);
    queue_test.enqueue(4);
    queue_test.enqueue(5);
    queue_test.enqueue(8);
    queue_test.enqueue(9);
    println!("{}", queue_test.print());
    queue_test.dequeue();
    queue_test.dequeue();
    queue_test.dequeue();
    println!("{}", queue_test.print());
    queue_test.enqueue(6);
    queue_test.enqueue(7);
    println!("{}", queue_test.print());

    let size_1: i32 = rand::thread_rng().gen_range(1..40);
    // let size_1 = 1;
    let mut stack_1: Stack = Stack::new((size_1 - 1) as u32);
    for _ in 1..size_1 {
        stack_1.push((random::<u32>() % 100 + 1) as i32);
    }
    let size_2: i32 = rand::thread_rng().gen_range(1..40);
    // let size_2 = 1;
    let mut stack_2: Stack = Stack::new((size_2 - 1) as u32);
    for _ in 1..size_2 {
        stack_2.push((random::<u32>() % 100 + 1) as i32);
    }
    let print1 = stack_1.print();
    println!("Stack 1 printout: {}", print1);
    let print2 = stack_2.print();
    println!("Stack 2 printout: {}", print2);


    let mut queue: Queue = Queue::new(size_1 as usize);

    // finding minimum
    let mut min: i32 = i32::MAX;
    while !stack_2.empty() {
        let temp = stack_2.pop().unwrap();
        if temp < min {
            min = temp
        }
    }
    println!("Min: {}", if min == i32::MAX {"".to_string()} else { format!("{}", min.to_string()) });

    while !stack_1.empty() {
        let i = stack_1.pop().unwrap();
        if min != 0 && i % min == 0 {
            queue.enqueue(i);
        }
    }
    let print3 = queue.print();
    println!("Queue: {}", print3);
}
