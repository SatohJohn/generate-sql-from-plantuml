#[cfg(test)]
mod tests {
    mod table {
        use crate::table::{column::Column, Table};

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
            let actual = Table::new(String::from(
                "entity piyo {
                hogehoge
            }",
            ));
            assert_eq!(true, actual.is_some());
            let a = actual.unwrap();
            assert_eq!("piyo", a.get_name());
            assert_eq!(
                &vec!(
                    Column::of(String::from("hogehoge")).unwrap() // ここはテストコードなのでいい
                ),
                a.get_content()
            );
        }

        #[test]
        fn table_content_is_multiline_with_none() {
            let actual = Table::new(String::from(
                "entity piyo {
                hogehoge
    
            }",
            ));
            assert_eq!(true, actual.is_some());
            let a = actual.unwrap();
            assert_eq!("piyo", a.get_name());
            assert_eq!(
                &vec!(
                    Column::of(String::from("hogehoge")).unwrap() // ここはテストコードなのでいい
                ),
                a.get_content()
            );
        }
    }

    mod column {
        use crate::table::column::Column;

        #[test]
        fn column_has_name_and_type() {
            let actual = Column::of(String::from("hogehoge piyo"));
            assert_eq!(true, actual.is_some());
            let a = actual.unwrap();
            assert_eq!("hogehoge", a.get_name());
            assert_eq!("piyo", a.get_type_definition());
        }

        // commentを使えるようにしたい
        #[test]
        fn column_has_name_and_type_def() {
            let actual = Column::of(String::from("hogehoge int(20) --comment"));
            assert_eq!(true, actual.is_some());
            let a = actual.unwrap();
            assert_eq!("hogehoge", a.get_name());
            assert_eq!("int(20)", a.get_type_definition());
        }
    }

    mod parser {
        use crate::table::parser;

        #[test]
        fn 文字列のスペースを区切り文字として認識して文字を分割する() {
            let input_data = String::from("hogehoge int(20) --comment");
            let actual = parser::line_format(&input_data);
            assert_eq!(vec!["hogehoge", "int(20)", "--comment"], actual);
        }
    }
}
