use regex::Regex;

pub struct Table {
    name: String,
    content: Vec<Column>
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
            content: content.split("\n").filter_map(|line| {
                if line.trim().len() == 0 {
                    return None
                }
                return Column::of(line.to_string());
            }).collect()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_content(&self) -> &Vec<Column> {
        &self.content
    }
}

#[derive(PartialEq, Debug)]
struct Column {
    name: String,
    type_definition: String
}

impl Column {
    /// constructor
    fn of(content: String) -> Option<Column> {
        let result = content.split(' ').filter_map(|c| {
            let trim = c.trim();
            if trim.is_empty() { None } else { Some(trim) }
        }).collect::<Vec<&str>>();
        let name = result.get(0).unwrap();
        let type_def = result.get(1).map(|s| s.to_string());
        return Some(Column {
            name: name.to_string(),
            type_definition: type_def.unwrap_or(String::from("")),
        });
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_type_definition(&self) -> String {
        self.type_definition.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::table::Table;
    use crate::table::Column;

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
        assert_eq!(&vec!(
            Column::of(String::from("hogehoge")).unwrap() // ここはテストコードなのでいい
        ), a.get_content());
    }

    #[test]
    fn table_content_is_multiline_with_none() {
        let actual = Table::new(String::from("entity piyo {
            hogehoge

        }"));
        assert_eq!(true, actual.is_some());
        let a = actual.unwrap();
        assert_eq!("piyo", a.get_name());
        assert_eq!(&vec!(
            Column::of(String::from("hogehoge")).unwrap() // ここはテストコードなのでいい
        ), a.get_content());
    }

    #[test]
    fn column_has_name_and_type() {
        let actual = Column::of(String::from("hogehoge piyo"));
        assert_eq!(true, actual.is_some());
        let a = actual.unwrap();
        assert_eq!("hogehoge", a.get_name());
        assert_eq!("piyo", a.get_type_definition());
    }
}
