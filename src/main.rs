#![allow(dead_code)]

mod list;
use list::List;

fn main() {
    let mut list = List::single(1);
    list.append(List::single(2));
    list.append(List::single(3));
    list.append(List::single(4));
    println!("List ({} elements): {}", list.length(), list);
    list.reverse();
    println!("Reversed: {}", list);
    list.drop_head();
    println!("Shorter list: {}", list);
}
