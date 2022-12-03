use crate::static_data::structs::{
    Body, Div, Doctype, Head, Require, StyleChild, StyleMain, StyleSheet, TermmlMain,
};

impl TermmlMain<'_> {
    pub fn parse_error<T: std::fmt::Display>(filename: &str, e: T) -> TermmlMain {
        TermmlMain {
            doctype: Doctype {
                ml: "termml".into(),
            },
            require: None, 
            head: Head {
                value: Div {
                    class: None,
                    value: "Error while parsing Termml file".into(),
                },
            },
            body: Body {
                value: vec![Div {
                    class: None,
                    value: format!("Message : in {} Error : {}", filename, e).into(),
                }],
            },
        }
    }
    pub fn fetch_error<T: std::fmt::Display>(url: &str, e: T, code: u16) -> TermmlMain {
        TermmlMain {
            doctype: Doctype {
                ml: "termml".into() 
            },
            require: None,
            head: Head {
                value: Div {
                    class: None,
                    value: "Error while fetching requested files".into()
                },
            },
            body: Body {
                value: vec![
                    Div{
                        class: None,
                        value: format!("Requested URL : {}", url).into()
                    },
                    Div {
                        class: None,
                        value: format!("Status Code : {}", code).into()
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
