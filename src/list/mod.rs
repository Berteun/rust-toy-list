use std::fmt;
use std::mem;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(data : T) -> Node<T> {
        Node{ data : data , next : None }
    }

    pub fn length(&self) -> usize {
        match *self {
            Node{data : _, next : Some(ref next)} => 1 + next.length(),
            Node{data : _, next : _}              => 1
        }
    }

    pub fn last(&self) -> &Node<T> {
        match *self {
            Node{data : _, next : Some(ref next)} => next.last(),
            Node{data : _, next : _}              => self
        }
    }

    pub fn last_mut(&mut self) -> &mut Node<T> {
        match *self {
            Node{data : _, next : Some(ref mut next)} => next.last_mut(),
            Node{data : _, next : _}                  => self
        }
    }

    pub fn append(&mut self, node: Node<T>) {
        self.last_mut().next = Some(Box::new(node));
    }

    pub fn prepend(&mut self, node: Node<T>) {
        let old_self = mem::replace(self, node);
        self.next = Some(Box::new(old_self));
    }

    /* One idiosyncracy is that we don't allow to drop the head of a singleton list. */
    pub fn drop_head(&mut self) -> Option<Box<Node<T>>> {
        let old_next = mem::replace(&mut self.next, None);
        match old_next {
            Some(boxed_node) => Some(Box::new(mem::replace(self, *boxed_node))),
            None             => None
        }
    }

    /* This is less elegant than I hoped; it suggests the proper way is a list class
     * wrapper; so you could start with an empty list and then remove elements one by
     * one until you exhaust the current list.
     */
    pub fn reverse(&mut self) {
        if self.next.is_none() {
            return;
        }

        let mut new_list = *self.drop_head().unwrap();
        while self.next.is_some() {
            new_list.prepend(*self.drop_head().unwrap())
        }
        let old_self = mem::replace(self, new_list);
        self.prepend(old_self);
    }
}

impl<T : fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node{data : _, next : Some(ref tail)} => write!(f, "{}->{}", self.data, tail),
            Node{data : _, next : _             } => write!(f, "{}"  , self.data)
        }
    }
}


#[test]
fn create_singleton_list() {
    let list = Node::new(1);
    assert!(list.data == 1);
    assert!(list.next.is_none());
}

#[test]
fn create_and_append() {
    let mut list = Node::new(1);
    list.append(Node::new(2));
    assert!(list.data == 1);
    assert!(list.next.is_some());
    assert!(list.next.unwrap().data == 2);
}

#[test]
fn create_and_prepend() {
    let mut list = Node::new(2);
    list.prepend(Node::new(1));
    assert!(list.data == 1);
    assert!(list.next.is_some());
    assert!(list.next.unwrap().data == 2);
}

#[test]
fn test_length() {
    let mut list = Node::new(1);
    assert!(list.length() == 1);
    list.append(Node::new(2));
    assert!(list.length() == 2);
}

#[test]
fn test_last_singleton() {
    let list = Node::new(1);
    assert!(list.last().data == 1);
}

#[test]
fn test_last() {
    let mut list = Node::new(1);
    list.append(Node::new(2));
    list.append(Node::new(3));
    list.append(Node::new(4));
    assert!(list.last().data == 4);
    assert!(list.length() == 4);
}

#[test]
fn test_last_prepend() {
    let mut list = Node::new(4);
    list.prepend(Node::new(3));
    list.prepend(Node::new(2));
    list.prepend(Node::new(1));
    assert!(list.last().data == 4);
    assert!(list.length() == 4);
}

#[test]
fn test_last_append_prepend() {
    let mut list = Node::new(2);
    list.prepend(Node::new(1));
    list.append(Node::new(3));
    list.append(Node::new(4));
    assert!(list.last().data == 4);
    assert!(list.length() == 4);
}

#[test]
fn test_reverse_singleton() {
    let mut list = Node::new(1);
    list.reverse();
    assert!(list.last().data == 1);
    assert!(list.length() == 1);
}

#[test]
fn test_reverse_longer() {
    let mut list = Node::new(1);
    list.append(Node::new(2));
    list.append(Node::new(3));
    assert!(list.data == 1);
    assert!(list.last().data == 3);
    assert!(list.length() == 3);
    list.reverse();
    assert!(list.data == 3);
    assert!(list.last().data == 1);
    assert!(list.length() == 3);
}

#[test]
fn test_drop() {
    let mut list = Node::new(1);
    list.append(Node::new(2));
    assert!(list.data == 1);
    assert!(list.length() == 2);
    let node = list.drop_head();
    assert!(node.unwrap().data == 1);
    assert!(list.data == 2);
    assert!(list.length() == 1);
}

#[test]
fn test_drop_singleton() {
    let mut list = Node::new(1);
    let node = list.drop_head();
    assert!(node.is_none());
    assert!(list.data == 1);
    assert!(list.length() == 1);
}
