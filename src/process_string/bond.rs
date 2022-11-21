use std::collections::{hash_map, HashMap};
use std::fs;
use smallvec::{SmallVec, smallvec};
use crate::static_data::structs::{StyleMain, StyleChild};
use ansi_term::Style;


fn styles_hash() -> HashMap<String, Style> {
    let mut map: HashMap<String, Style> = HashMap::new();
    for i in get_style_sheets("./".to_string()) {
        
    }
    map
}

fn parse_style_sheet(file: &str) -> StyleMain {
    match toml::from_str(file) {
        Ok(p) => {
            p
        }
        Err(e) => {
            StyleMain { styles: vec![ StyleChild{
                tag: String::from("null"),
                background: None,
                foreground: None,
                wrap: None,
                margin: None
            }]}
        }
    }
}
//get all files with .termss extension
//std::fs sorts files by alphabetical order not folder>files
pub fn get_style_sheets(start: String) -> Vec<String> {
    //gotta fix unwraps later
    let paths = fs::read_dir(&start).unwrap();
    for path in paths {
        println!("path : {}", 
            path.as_ref().unwrap().path().display());
    }
    //thing
    // let s = start.split("");
    // let mut arr =
    //     s.map(String::from).collect::<Vec<String>>();
    // for _ in 0..start.len() {
    //     arr.remove(0);
    // }
    // let start = arr.join("");
    // println!("start: {}", start);
    let paths = fs::read_dir(start).unwrap();
    println!("recursion starts here");
    for path in paths {
        // let to_get = start.clone()+"|"+&path.unwrap().path().display().to_string();
        let to_get = path.unwrap().path().display().to_string();

        println!("toget: {}", to_get);
        get_style_sheets(to_get);
    }
    vec![]
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
