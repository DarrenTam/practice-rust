
pub(crate) struct Node {
    pub(crate) value: i32,
    pub(crate) next: Option<Box<Node>>,
}

impl Node {
    pub fn get_next(self) -> Option<Box<Node>> {
        self.next
    }
    pub fn get_value(self) -> i32 {
        self.value
    }
    pub fn set_next(&mut self, node: Option<Box<Node>>) {
        self.next = node;
    }
    fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}
