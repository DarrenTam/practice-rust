use std::fmt;

use crate::node::Node;

mod node;
mod linked_list;

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
    linked_list.insert(1,765);
    linked_list.insert(2,999);
    println!("after insert");
    linked_list.print_all();
    linked_list.erase(1);
    println!("after earse");
    linked_list.print_all();
    linked_list.reverse();
    println!("after reverse");
    linked_list.print_all();
    linked_list.remove_value(999);
    println!("after remove value");
    linked_list.print_all();
}