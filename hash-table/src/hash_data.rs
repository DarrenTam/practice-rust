#[derive(Default, Clone)]
pub(crate) struct HashData {
    pub(crate) key: String,
    pub(crate) value: String,
}

impl HashData{
    pub fn new() -> HashData{
        HashData{
            key: String::from(""),
            value: String::from("")
        }
    } 
}