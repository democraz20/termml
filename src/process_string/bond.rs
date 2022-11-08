use std::collections::HashMap;
use crate::process_string::serialize::{
    IndexMain, StyleMain, StyleChild
};

fn bond_termml(content: IndexMain, style: StyleMain) -> (){
    //need to change this later to associated type
    let mut map: HashMap<String, String> = HashMap::new();
    for i in 0..content.body.len(){

    }
}
fn find_style_by_tag(find: String, from: StyleMain) -> StyleChild {
    //optimization LATER
    for i in from.styles{
        if i.tag == find {
            return i;
        }
    }

    return StyleChild {
        tag: "NULL".to_string(),
        background: None,
        foreground: None,
        wrap: None,
        margin: None,
    }
    //for when no match is found
}