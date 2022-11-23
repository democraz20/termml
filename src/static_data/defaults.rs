use crate::static_data::structs::{
    StyleMain,
    StyleChild,
    TermmlMain,
    Doctype,
    Head, Body, Div,
    StyleSheet,
    Require
};

impl TermmlMain<'_> {
    pub fn new_error<T: std::fmt::Display>(filename: &str, e: T) -> TermmlMain {
        TermmlMain {
            doctype: Doctype {ml: "termml".into()},
            require: Some(Require {
                stylesheet: vec![
                    StyleSheet { name: Some("styles.termss".into())}
                ]
            }),
            head: Head {
                value: Div {
                    class: None,
                    value: "Error while parsing Termml file".into()
                },
            },
            body: Body {
                value: vec![
                    Div {
                        class: None,
                        value: format!("Message : in {} Error : {}", filename, e).into()
                    }
                ]
            }
        }
    }
}

// use ansi_term::{Colour};

// pub const DEFAULT_ML_COLOR: Colour = Colour::White;

// impl IndexMain {
//     pub fn new_error<T: std::fmt::Display>(filename: &str, e: T) -> IndexMain {
//         IndexMain {
//             doctype: String::from("TERMML"),
//             head: IndexChild {
//                 tag: String::from("head"),
//                 value: String::from("Error : Failed to parse termML file"),
//                 class: None,
//                 link: None
//             },
//             stylesheet: None,
//             body: vec![
//                 IndexChild {
//                     tag: String::from("body"),
//                     value: String::from(format!("Filename: {}\nInfo : {}", filename, e)),
//                     class: None,
//                     link: None
//                 }
//             ]
//         }
//     }
// }
