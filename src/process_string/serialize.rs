use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct TOMLmain {
    doctype: String,
    head: TOMLchild,
    body: Vec<TOMLchild>,
}

#[derive(Deserialize)]
pub struct TOMLchild {
    tag: String,
    value: String,
    class: Option<String>,
    link: Option<String>
}