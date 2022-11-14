use serde_derive::Deserialize;
use std::borrow::Cow;
use strong_xml::{XmlRead};
//main markup
#[derive(XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "termml")]
pub struct IndexMain<'a> {
    #[xml(text)]
    pub doctype: Cow<'a, str>,
    #[xml(text)]
    pub stylesheet: Option<Cow<'a, str>>,
    #[xml(child = "div")]
    pub head: IndexChild<'a>,
    #[xml(child = "div")]
    pub body: IndexChild<'a>,
}

#[derive(XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "div")]
pub struct IndexChild<'a> {
    #[xml(attr = "class")]
    pub class: Option<Cow<'a, str>>,
    #[xml(attr = "link")]
    pub link: Option<Cow<'a, str>>,
    #[xml(text)]
    pub value: Cow<'a, str>,
}

//styles
#[derive(Debug, Deserialize)]
pub struct StyleMain {
    pub styles: Vec<StyleChild>
}

#[derive(Default, Debug, Deserialize)]   
pub struct StyleChild {
    pub tag: String,
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub wrap: Option<u16>,
    pub margin: Option<u8>,
}