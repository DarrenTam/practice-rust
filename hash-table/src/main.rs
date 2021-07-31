use crate::hash_table::HashTable;
use crate::hash_data::HashData;

mod hash_data;
mod hash_table;

fn main() {
    let mut hash_table = hash_table::HashTable::new(30);
    hash_table.add(String::from("22222"), String::from("dsdsadsadsa"));
    hash_table.add(String::from("22224"), String::from("dabcc"));
    println!("{}", hash_table.get(String::from("22222")));
    println!("{}", hash_table.get(String::from("22224")));
    println!("{}", hash_table.exist(String::from("aaa")));
    println!("{}", hash_table.exist(String::from("22222")));
    hash_table.remove(String::from("22222"));
    println!("{}", hash_table.exist(String::from("22222")));
}