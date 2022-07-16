pub struct Table {
    name: String,
    content: String
}

impl Table {
    pub fn new(content: String) -> Table {
        Table { name: String::from("hoge"), content: content }
    }

    pub fn get_name(self) -> String {
        self.name
    }
}