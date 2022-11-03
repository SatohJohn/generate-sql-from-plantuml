
#[cfg(test)]
mod tests {

    mod table {
        use crate::table::{Table, column::Column};

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

    }

}
