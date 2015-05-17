#![allow(dead_code)]

mod list;
use list::Node;

fn main() {
    let mut list = Node::new(1);
    list.append(Node::new(2));
    list.append(Node::new(3));
    list.append(Node::new(4));
    println!("List ({} elements): {}", list.length(), list);
    list.reverse();
    println!("Reversed: {}", list);
}
