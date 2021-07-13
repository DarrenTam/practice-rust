mod node;
mod linked_list;

use std::fmt;
use crate::node::Node;

fn main() {
    let head =
        Some(Box::new(node::Node {
            value: 2,
            next:None,
        }));

    let mut linked_list = Box::new(linked_list::LinkedList {
        head
    });

    linked_list.push_back(10);
    linked_list.push_back(40);
    linked_list.print_all();
    println!("{}",linked_list.valued_at(1));
    println!("{}",linked_list.empty());
    println!("{}",linked_list.size());
    linked_list.push_front(99);
    linked_list.pop_front();
    linked_list.print_all();
    linked_list.pop_back();
    println!("after pop back");
    linked_list.print_all();
}