use std::{
    fs, collections::HashMap
};
use crate::static_data::structs::{
    StyleMain,
    StyleChild,
};
use ansi_term::Style;

pub fn styles_hash() -> HashMap<String, StyleChild> {

    //imagine a getter from Require termml page
    let mut stylesmap: HashMap<String, StyleChild> = HashMap::new();
    let required = vec![
        String::from("styles.termss")
    ];
    
    for i in required {
        let file = fs::read_to_string(i.as_str()).unwrap();
    
        let s = i.split(".");
        let mut t = s.clone().map(String::from).collect::<Vec<String>>();
        t.pop();
        let styles_namespace = t.join("."); //splitted ".", rejoin "."
        // let styles_namespace = t[t.len()-1].clone();
        drop(s);
        drop(t);
    
        let styles = parse_style_sheet(file);
        for i in styles.styles {
            stylesmap.insert(
                format!("{}::{}", styles_namespace, i.class.clone()),
                // i.class.clone(),
                i
            );
        }
    }
    
    return stylesmap;
}

pub fn parse_style_sheet(file: String) -> StyleMain {
    let mut file = file.replace("\n", "");
    file = file.replace("\r", "");

    match toml::from_str(&file) {
        Ok(p) => {
            p
        }
        Err(e) => {
            println!("error while parsing: {}", e);
            StyleMain { styles: vec![ StyleChild{
                class: String::from("null"),
                background: None,
                foreground: None,
                underline: None,
                bold: None,
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
