use regex::Regex;

pub struct Table {
    name: String,
    content: String
}

impl Table {
    
    pub fn new(content: String) -> Option<Table> {
        let re = Regex::new(r"([a-zA-Z]+) ([a-zA-Z]+) (.+)").unwrap();
        return re.captures(content.as_str())
            .map(|captured| {
                return Table {
                    name: captured.get(2).map_or(String::from(""), |m| m.as_str().to_string()),
                    content: captured.get(3).map_or(String::from(""), |m| m.as_str().to_string())
                }
            });
    }

    pub fn get_name(self) -> String {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::table::Table;

    #[test]
    fn table_name_is_hoge() {
        let actual = Table::new(String::from("entity hoge { }"));
        assert_eq!(true, actual.is_some());
        assert_eq!("hoge", actual.unwrap().get_name());
    }

    #[test]
    fn table_name_is_fuga() {
        let actual = Table::new(String::from("entity fuga { }"));
        assert_eq!(true, actual.is_some());
        assert_eq!("fuga", actual.unwrap().get_name());
    }
}
