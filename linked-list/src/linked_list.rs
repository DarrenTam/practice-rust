use std::mem;
use std::ops::Deref;

use crate::node::Node;

pub(crate) struct LinkedList {
    pub(crate) head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn size(&self) -> i32 {
        let mut size = 1;
        match self.head {
            Some(ref head) => {
                let mut current = head;
                while current.next.is_some() {
                    size += 1;
                    current = current.next.as_ref().unwrap();
                }
            }
            _ => {}
        }
        return size;
    }

    pub fn empty(&self) -> bool {
        return self.head.is_none();
    }

    pub fn valued_at(&self, index: i32) -> i32 {
        let mut value = -1;
        match self.head {
            Some(ref head) => {
                let mut current = head;
                for i in 0..index {
                    current = current.next.as_ref().unwrap();
                }
                value = current.value;
            }
            _ => {}
        }
        return value;
    }

    pub fn push_front(&mut self, value: i32) {
        let mut node = Node {
            value,
            next: mem::replace(&mut self.head, None),
        };
        self.head = Some(Box::new(node));
    }

    pub fn pop_front(&mut self) {
        let temp = mem::take(&mut self.head.as_mut().unwrap().next);
        self.head = temp;
    }

    pub fn lenght(&self) -> i32 {
        let mut count = 0;
        let mut current = self.head.as_ref();
        while current.is_some() {
            current = current.unwrap().next.as_ref();
            count += 1;
        }
        count
    }

    //Overhead way to do this, O(2N), or just store the length in the list, it could be O(N)
    pub fn pop_back(&mut self) {
        let length = self.lenght() - 2;
        let mut current = self.head.as_mut();
        for _x in 0..length {
            current = current.unwrap().next.as_mut()
        }
        current.as_mut().unwrap().set_next(None);
    }

    pub fn push_back(&mut self, value: i32) {
        let node = Node {
            value,
            next: None,
        };
        match self.head {
            None => {
                self.head = Some(Box::from(node))
            }
            Some(ref mut head) => {
                let mut currrent = head;
                while currrent.next.is_some() {
                    currrent = currrent.next.as_mut().unwrap();
                }
                currrent.next = Some(Box::new(node));
            }
        }
    }

    pub fn front(&self) -> i32 {
        return self.head.as_ref().unwrap().value;
    }

    pub fn back(&mut self) -> i32 {
        let mut value = 0;
        match self.head {
            Some(ref head) => {
                let mut current = head;
                while current.next.is_some() {
                    current = current.next.as_ref().unwrap();
                }
                value = current.value;
            }
            _ => {}
        }
        return value;
    }

    //O(2N) soultion,
    pub fn insert(&mut self, index : i32, vaule: i32){
        let mut next_node =  self.head.as_mut();

        for _i in 0..index-1 {
            next_node = next_node.unwrap().next.as_mut()
        }

        let mut node = Node {
            value:vaule,
            next:  None
        };

        node.set_next(next_node.unwrap().next.take());

        let mut current = self.head.as_mut();
        for _i in 0..index-1 {
            current = current.unwrap().next.as_mut()
        }

        current.unwrap().next = Some(Box::new(node));
    }

    fn split_node(&mut self, index: i32) -> Option<Box<Node>> {
        let mut current =  self.head.as_mut();
        for _i in 0..index {
            current = current.unwrap().next.as_mut();
        }
        current.take().map(|node| node.next.take()).unwrap()
    }

    //O(2N) solution
    pub fn erase(&mut self, index: i32) {

        let node_after_index =  self.split_node(index);

        let mut current =  self.head.as_mut();
        for _i in 0..index-1 {
            current = current.unwrap().next.as_mut();
        }

        current.unwrap().set_next(node_after_index);
    }

    //TODO Bit ugly refactor this
    pub fn reverse(&mut self) {
        let mut current = self.head.take();
        let mut prev = None ;
        while current.is_some() {
            let mut current_unwraped = current.take().unwrap();
            let next = current_unwraped.next.take();
            current_unwraped.next = prev.take();
            prev = Some(current_unwraped);
            current = next;
        }
        self.head = prev;
    }

    pub fn remove_value(&mut self, value: i32) {
        let mut i = 0;
        match self.head {
            Some(ref mut head) => {
                let mut current = head;
                while current.next.is_some() {
                    if current.value==value {
                        self.erase(i);
                        break;
                    }
                    i+=1;
                    current = current.next.as_mut().unwrap();
                }
            }
            _ => {}
        }
    }

    pub fn print_all(&self) {
        match self.head {
            Some(ref head) => {
                let mut current = head;
                println!("//---------//");
                while current.next.is_some() {
                    println!("value: {}", current.as_ref().value);
                    current = current.next.as_ref().unwrap();
                }
                println!("value: {}", current.as_ref().value);
            }
            _ => {}
        }
    }
}