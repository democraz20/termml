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
}
