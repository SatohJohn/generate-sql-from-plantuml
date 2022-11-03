
#[derive(PartialEq, Debug)]

pub struct Column {
    name: String,
    type_definition: String
}

impl Column {
    /// constructor
    pub fn of(content: String) -> Option<Column> {
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