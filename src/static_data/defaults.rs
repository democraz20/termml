use crate::static_data::structs::{
    IndexMain,
    IndexChild,
};

impl IndexMain {
    pub fn new_error<T: std::fmt::Display>(filename: &str, e: T) -> IndexMain {
        IndexMain {
            doctype: String::from("TERMML"),
            head: IndexChild {
                tag: String::from("head"),
                value: String::from("Error : Failed to parse termML file"),
                class: None,
                link: None
            },
            stylesheet: None,
            body: vec![
                IndexChild {
                    tag: String::from("body"),
                    value: String::from(format!("Filename: {}\nInfo : {}", filename, e)),
                    class: None,
                    link: None
                }
            ]
        }
    }
}
