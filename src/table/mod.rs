use regex::Regex;

pub mod column;
pub mod parser;

pub struct Table {
    name: String,
    content: Vec<column::Column>,
}

impl Table {
    /// Tableを作成し返却する
    /// # Arguments
    /// - `content`: entity全体
    pub fn new(content: String) -> Option<Table> {
        let re = Regex::new(r"(entity|class) (\w+) \{([\s\S]*)\}").unwrap();
        return re.captures(content.as_str()).map(|captured| {
            return Table::of(
                captured
                    .get(2)
                    .map_or(String::from(""), |m| m.as_str().to_string()),
                captured
                    .get(3)
                    .map_or(String::from(""), |m| m.as_str().to_string()),
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
            content: content
                .split("\n")
                .filter_map(|line| {
                    if line.trim().len() == 0 {
                        return None;
                    }
                    return column::Column::of(line.to_string());
                })
                .collect(),
        };
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_content(&self) -> &Vec<column::Column> {
        &self.content
    }

    pub fn convert_to_sql(&self) -> String {
        String::from("")
    }
}
