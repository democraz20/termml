use serde_derive::Deserialize;
use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};
//main markup
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "main")]
pub struct TermmlMain<'a> {
    #[xml(child = "doctype")]
    pub doctype: Doctype<'a>,
    #[xml(child = "require")]
    // pub require: Require<'a>,
    pub require: Option<Require<'a>>,
    #[xml(child = "head")]
    pub head: Head<'a>,
    #[xml(child = "body")]
    pub body: Body<'a>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "doctype")]
pub struct Doctype<'a> {
    #[xml(attr = "ml")]
    pub ml: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "head")]
pub struct Head<'a> {
    #[xml(child = "div")]
    pub value: Div<'a>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "body")]
pub struct Body<'a> {
    #[xml(child = "div")]
    pub value: Vec<Div<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "div")]
pub struct Div<'a> {
    #[xml(attr = "class")]
    pub class: Option<Cow<'a, str>>,
    #[xml(text)]
    pub value: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "require")]
pub struct Require<'a> {
    #[xml(child = "Stylesheet")]
    pub stylesheet: Vec<StyleSheet<'a>>
}
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "Stylesheet")]
pub struct StyleSheet<'a> {
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>
}

//to be done with YAML or XML
//styles
#[derive(Debug, Deserialize)]
pub struct StyleMain {
    pub styles: Vec<StyleChild>,
}

#[derive(Default, Debug, Deserialize)]
pub struct StyleChild {
    pub tag: String,
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub wrap: Option<u16>,
    pub margin: Option<u8>,
}
