use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping node with value {}", self.value);
    }
}

fn main() {
    let tail = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
    });

    let head = Rc::new(Node {
        value: 2,
        next: RefCell::new(Some(tail.clone())),
    });

    // 循环引用导致内存永远不释放
    // *tail.next.borrow_mut() = Some(head.clone());

    println!("head: {head:?}");
}
