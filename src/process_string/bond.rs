use std::collections::{hash_map, HashMap}; 
use crate::static_data::structs::{
    StyleMain,
    StyleChild,
};
use ansi_term::Style;

fn styles_hash() -> HashMap<String, Style> {
    let mut map: HashMap<String, Style> = HashMap::new();
    map
}

pub fn parse_style_sheet(file: String) -> StyleMain {
    let mut file = file.replace("\n", "");
    file = file.replace("\r", "");

    match toml::from_str(&file) {
        Ok(p) => {
            p
        }
        Err(_) => {
            StyleMain { styles: vec![ StyleChild{
                class: String::from("null"),
                background: None,
                foreground: None,
                wrap: None,
                margin: None
            }]}
        }
    }
}
// fn bond_termml(content: IndexMain, style: StyleMain) -> (){
//     //need to change this later to associated type
//     for i in 0..content.body.len(){

//     }
// }
// fn find_style_by_tag(find: String, from: StyleMain) -> StyleChild {
//     //optimization LATER
//     for i in from.styles{
//         if i.tag == find {
//             return i;
//         }
//     }

//     return StyleChild {
//         tag: "None".to_string(),
//         background: None,
//         foreground: None,
//         wrap: None,
//         margin: None,
//     }
//     //for when no match is found
// }
