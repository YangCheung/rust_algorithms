use std::{rc::Rc, cell::RefCell};

struct Node<T> {
    next: Option<Rc<RefCell<Box<Node<T>>>>>,
    value: T,
}


fn make_chain(head: Rc<RefCell<Box<Node<i32>>>>) {
    let mut n = 10;
    let mut temp_head = head;
    while n > 0 {
        let next = Rc::new(RefCell::new(Box::new(Node{ next: None, value: n})));
        print!("make next value is {} \r\n", next.borrow().as_ref().value);
        temp_head.clone().borrow_mut().next = Some(next.clone());
        temp_head = next;
        n -= 1;
    }
}

fn print_chain(head: Option<Rc<RefCell<Box<Node<i32>>>>>) {
    let mut temp_head = head;
    while let Some(real_next) = temp_head  {
        print!("next value is {}\r\n", real_next.clone().borrow().value);
        temp_head = real_next.borrow().next.clone();
    }
}

pub fn test() {
    let head = Box::new(Node{ next: None, value: 1});
    let head = Rc::new(RefCell::new(head));
    make_chain(head.clone());
    print_chain(Some(head.clone()))
}