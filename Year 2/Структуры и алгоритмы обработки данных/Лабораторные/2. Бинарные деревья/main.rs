use rand::Rng;  
use crate::Order::In;  
  
#[derive(Debug)]  
struct Node {  
    value: i32,  
    left: Link,  
    right: Link  
}  
type Link = Option<Box<Node>>; 
#[derive(Clone, Copy)]  
enum Order {  
    Pre,  
    In,  
    Post,  
    InClear,  
}  
fn count_entries(node: Node, value: i32) -> i32 {  
    let mut sum = 0;  
    if node.value == value {  
        sum += 1;  
    }    if let Some(l) = node.left {  
        sum += count_entries(*l, value)  
    }    if let Some(r) = node.right {  
        sum += count_entries(*r, value)  
    }    sum  
}   
impl Node {  
    fn new(value: i32) -> Self {  
        Node {  
            value,  
            left: None,  
            right:None  
        }  
    }    fn insert(&mut self, value: i32) {  
        let target_node = if rand::random() {&mut self.left} else {&mut self.right};  
        match target_node {  
            None => {  
                let new_node = Box::new(Node::new(value));  
                *target_node = Some(new_node);  
            }            Some(node) => {  
                node.insert(value);  
            }        }    }    fn traverse(&self, order: Order) {  
        if let Order::Pre = order {  
            print!("{:?} ", self.value);  
        }        if let Some(l) = &self.left {  
            l.traverse(order);  
        }        if let Order::In = order {  
            print!("{:?} ", self.value);  
        }        if let Some(r) = &self.right {  
            r.traverse(order);  
        }        if let Order::Post = order {  
            print!("{:?} ", self.value);  
        }    }}  
fn main() {  
    let mut node: Node = Node::new(rand::thread_rng().gen_range(1..20));  
    let lim: i32 = rand::thread_rng().gen_range(10..15);  
    for i in 0..lim {  
        node.insert(rand::thread_rng().gen_range(1..50));  
    }    
    print!("\nПрямой: ");  
    node.traverse(Order::Pre);  
    print!("\nСимметричный: ");  
    node.traverse(Order::In);  
    print!("\nОбратный: ");  
    node.traverse(Order::Post);  
    let value = 1;  
    println!("\nEntries of {value}: {}", count_entries(node, 1))  
}
