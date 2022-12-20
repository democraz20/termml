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
    pub fn fetch_error<T: std::fmt::Display>(
        url: &str,
        e: Option<T>,
        code: Option<u16>,
    ) -> TermmlMain {
        TermmlMain {
            doctype: Doctype {
                ml: "termml".into(),
            },
            require: None,
            head: Head {
                value: Div {
                    class: None,
                    value: "Error while fetching requested files".into(),
                },
            },
            body: Body {
                value: vec![
                    Div {
                        class: None,
                        value: format!("Requested URL : {}", url).into(),
                    },
                    Div {
                        class: None,
                        value: match code {
                            Some(c) => format!("Status code: {}", c).into(),
                            None => format!("Unknown Status code").into(),
                        },
                    },
                    Div {
                        class: None,
                        value: match e {
                            Some(r) => r.to_string().into(),
                            None => String::from("An unknown error has occured").into(),
                        },
                    },
                ],
            },
        }
    }
}
