use std::fmt;
use std::mem;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

pub struct List<T> {
    head: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(data : T) -> Node<T> {
        Node{ data : data , next : None }
    }

    fn length(&self) -> usize {
        match *self {
            Node{data : _, next : Some(ref next)} => 1 + next.length(),
            Node{data : _, next : _}              => 1
        }
    }

    fn last(&self) -> &Node<T> {
        match *self {
            Node{data : _, next : Some(ref next)} => next.last(),
            Node{data : _, next : _}              => self
        }
    }

    fn last_mut(&mut self) -> &mut Node<T> {
        match *self {
            Node{data : _, next : Some(ref mut next)} => next.last_mut(),
            Node{data : _, next : _}                  => self
        }
    }

    fn append(&mut self, node: Node<T>) {
        self.last_mut().next = Some(Box::new(node));
    }

    fn prepend(&mut self, node: Node<T>) {
        let old_self = mem::replace(self, node);
        self.next = Some(Box::new(old_self));
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List{head : None}
    }

    pub fn single(data : T) -> List<T> {
        List{head : Some(Box::new(Node{ data : data, next : None}))}
    }

    pub fn length(&self) -> usize {
        match self.head {
            Some(ref boxed_node) => boxed_node.length(),
            None => 0
        }
    }

    pub fn first(&self) -> Option<&Node<T>>
    {
        match self.head
        {
            None => None,
            Some(ref boxed_node) => Some(&*boxed_node)
        }
    }

    pub fn last(&self) -> Option<&Node<T>>
    {
        match self.head {
            Some(ref boxed_node) => Some(boxed_node.last()),
            None => None
        }
    }

    pub fn append(&mut self, list: List<T>) {
        match self.head {
            Some(ref mut boxed_node) => boxed_node.last_mut().next = list.head,
            None => self.head = list.head
        }
    }

    pub fn prepend(&mut self, mut list: List<T>) {
        mem::swap(self, &mut list);
        self.append(list);
    }

    pub fn drop_head(&mut self) -> Option<List<T>> {
        let old_head = mem::replace(&mut self.head, None);
        match old_head {
            None => None,
            Some(mut boxed_node) => {
                self.head = mem::replace(&mut boxed_node.next, None);
                Some(List{head : Some(boxed_node)})
            }
        }
    }

    pub fn reverse(&mut self) {
        let mut new_list = List::new();
        while self.head.is_some() {
            new_list.prepend(self.drop_head().unwrap())
        }
        mem::replace(self, new_list);
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

impl<T : fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.head {
            Some(ref boxed_node) => write!(f, "{}", *boxed_node),
            None => write!(f, "<Empty>")
        }
    }
}


#[test]
fn create_node() {
    let list = Node::new(1);
    assert!(list.data == 1);
    assert!(list.next.is_none());
}

#[test]
fn create_node_and_append() {
    let mut list = Node::new(1);
    list.append(Node::new(2));
    assert!(list.data == 1);
    assert!(list.next.is_some());
    assert!(list.next.unwrap().data == 2);
}

#[test]
fn create_node_and_prepend() {
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
    let mut list = List::single(1);
    list.reverse();
    assert!(list.last().unwrap().data == 1);
    assert!(list.length() == 1);
}

#[test]
fn test_reverse_longer() {
    let mut list = List::single(1);
    list.append(List::single(2));
    list.append(List::single(3));
    assert!(list.first().unwrap().data == 1);
    assert!(list.last().unwrap().data == 3);
    assert!(list.length() == 3);
    list.reverse();
    assert!(list.first().unwrap().data == 3);
    assert!(list.last().unwrap().data == 1);
    assert!(list.length() == 3);
}

#[test]
fn test_drop() {
    let mut list = List::single(1);
    list.append(List::single(2));
    assert!(list.first().unwrap().data == 1);
    assert!(list.length() == 2);
    let single_list = list.drop_head();
    assert!(single_list.unwrap().first().unwrap().data == 1);
    assert!(list.first().unwrap().data == 2);
    assert!(list.length() == 1);
}

#[test]
fn test_drop_singleton() {
    let mut list = List::single(1);
    let node = list.drop_head();
    assert!(node.unwrap().first().unwrap().data == 1);
    assert!(list.first().is_none());
    assert!(list.length() == 0);
}
