use crate::hash_data::HashData;


pub(crate) struct HashTable {
    pub(crate) data: Vec<HashData>,
    pub(crate) len: i32,
}

impl HashTable {
    pub(crate) fn new(len: i32) -> HashTable {
        HashTable {
            data: vec![HashData::new(); len as usize],
            len,
        }
    }

    pub fn hash(&self, key: &String) -> usize {
        let mut hash = 0;
        let key_length = key.len() as i32;

        for i in 0..key_length {
            hash = hash * 31 + i;
        }

        return (hash % self.len).abs() as usize;
    }

    pub fn add(&mut self, key: String, value: String) {
        let mut index = self.hash(&key);

        if !self.data[index].key.eq(&key) {
            while !self.data[index].key.is_empty() {
                index = (index + 1) % self.len as usize;
            }
        }

        self.data[index].value = value;
        self.data[index].key = key;
    }

    pub fn get(&self, key: String) -> &str {
        let mut index = self.hash(&key);
        if self.data[index].key.eq(&key) {
            return self.data[index].value.as_ref();
        } else {
            while index < self.len as usize {
                if self.data[index].key.eq(&key) {
                    break;
                } else {
                    index = (index + 1) % self.len as usize;
                }
            }
        }
        return self.data[index].value.as_ref();
    }

    pub fn exist(&self, key: String) -> bool {
        let mut exist = false;
        let mut index = self.hash(&key);
        let original_index = index;

        if self.data[index].key.eq(&key) {
            exist = true;
            return exist;
        }

        while !self.data[index].key.eq(&key) {
            if self.data[index].key.eq(&key) {
                exist = true;
                break;
            } else {
                index = (index + 1) % self.len as usize;
            }

            if index == original_index {
                break;
            }
        }

        return exist;
    }

    pub fn remove(&mut self, key: String) {
        let mut index = self.hash(&key);

        if self.data[index].key.is_empty() {
            return;
        }

        if self.data[index].key.eq(&key) {
           self.data[index].key = String::from("");
        } else {
            while index < self.len as usize {
                if self.data[index].key.eq(&key) {
                    break;
                } else {
                    index = (index + 1) % self.len as usize;
                }
            }
        }
        return self.data[index].key= String::from("");
    }
}