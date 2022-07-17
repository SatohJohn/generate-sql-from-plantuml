use regex::Regex;

pub struct Table {
    name: String,
    content: String
}

impl Table {
    
    /// Tableを作成し返却する
    /// # Arguments
    /// - `content`: entity全体
    pub fn new(content: String) -> Option<Table> {
        let re = Regex::new(r"(entity|class) (\w+) \{([\s\S]*)\}").unwrap();
        return re.captures(content.as_str())
            .map(|captured| {
                return Table::of(
                    captured.get(2).map_or(String::from(""), |m| m.as_str().to_string()),
                    captured.get(3).map_or(String::from(""), |m| m.as_str().to_string())
                );
            });
    }
    
    /// Tableを作成し返却する
    /// # Arguments
    /// - `name`: entity name
    /// - `content`: entityの中身
    pub fn of(name: String, content: String) -> Table {
        return Table {
            name,
            content
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
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

    #[test]
    fn table_name_is_numeric() {
        let actual = Table::new(String::from("entity fuga13 { }"));
        assert_eq!(true, actual.is_some());
        assert_eq!("fuga13", actual.unwrap().get_name());
    }

    #[test]
    fn table_content_is_multiline() {
        let actual = Table::new(String::from("entity piyo {
            hogehoge
        }"));
        assert_eq!(true, actual.is_some());
        let a = actual.unwrap();
        assert_eq!("piyo", a.get_name());
        assert_eq!("
            hogehoge
        ", a.get_content());
    }
}
