#[derive(PartialEq, Debug)]

pub struct Column {
    name: String,
    type_definition: String,
}

// どの順番で入っているか
const NAME_INDEX: usize = 0;
const DEF_INDEX: usize = 1;

#[path = "parser/mod.rs"]
mod parser;

impl Column {
    /// constructor
    pub fn of(content: String) -> Option<Column> {
        // とりあえずスペース区切りでデータが入るはずである
        let result = parser::line_format(&content);

        let name = result.get(NAME_INDEX).unwrap();
        let type_def = result.get(DEF_INDEX).map(|s| s.to_string());

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

// #[derive(PartialEq, Debug)]
// pub struct ColumnDefinition {
//     type_name: String,
//     comment: String
// }

// impl ColumnDefinition {
//     pub fn of(content: String) -> Option<ColumnDefinition> {
//         let result = content.split(' ').filter_map(|c| {
//             let trim = c.trim();
//             if trim.is_empty() { None } else { Some(trim) }
//         }).collect::<Vec<&str>>();

//         let name = result.get(NAME_INDEX).unwrap();
//         let type_def = result.get(DEF_INDEX).map(|s| s.to_string());

//         return Some(Column {
//             name: name.to_string(),
//             type_definition: type_def.unwrap_or(String::from("")),
//         });
//     }
// }
