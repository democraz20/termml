use crate::static_data::structs::{Body, Div, Doctype, Head, TermmlMain};

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
    //should be moved to renderer crate
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
                    Div {
                        class: None,
                        value: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".into()
                    }
                ],
            },
        }
    }
}
