// use crate::static_data::structs::{
//     IndexMain, StyleMain, StyleChild
// };

use std::collections::{hash_map, HashMap};
use ansi_term::Style;

fn styles_hash() -> HashMap<String, Style> {
    let mut map: HashMap<String, Style> = HashMap::new();
    map
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
