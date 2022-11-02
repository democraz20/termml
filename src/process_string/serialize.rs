pub struct JSONmain {
    doctype: String,
    head: JSONchild,
    body: Vec<JSONchild>,
}

pub struct JSONchild {
    tag: String,
    text: String,
    class: String,
    link: String,
}