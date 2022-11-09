use serde_derive::Deserialize;

//main markup
#[derive(Default, Debug, Deserialize)]
pub struct IndexMain {
    pub doctype: String,
    pub stylesheet: Option<String>,
    pub head: IndexChild,
    pub body: Vec<IndexChild>,
}

#[derive(Default, Debug, Deserialize)]
pub struct IndexChild {
    pub tag: String,
    pub value: String,
    pub class: Option<String>,
    pub link: Option<String>
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