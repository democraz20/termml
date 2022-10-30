use smallvec::SmallVec;

pub fn split_tags(text: String) -> Vec<String> {
    let text = collapse_file(text);
    let split = text.split("><");
    let mut untreated_list = split.map(String::from).collect::<Vec<String>>();
    if untreated_list.len() >= 3 {
        //reserve
        let len: usize = untreated_list.len();
        let first = untreated_list[0].clone()+">";
        let last = "<".to_string()+&untreated_list[len-1].clone();
        // println!("first:'{}' second:'{}'", first, last);
        //remove the first and last items so things can iterate flawlessly
        for i in 0..len { 
            //loop with the number of items instead
            untreated_list[i] = "<".to_string()+&untreated_list[i]+&">".to_string();
        } 
        untreated_list[0] = first.clone();
        drop(first);
        untreated_list[len-1] = last.clone();
        drop(last);
        drop(len);
        return untreated_list;
    } else if untreated_list.len() == 2 {

    } else {
        //reserve for no element
    }
    //list will never have 1 item in it
    //either more or less (none or more)
    return vec![]
}

fn collapse_file(text: String) -> String {
    //assuming text has no whitespaces\unwanted elements
    let split = text.split("\n");
    let untreated_list = split.map(String::from).collect::<Vec<String>>();



    let mut final_text: Vec<String> = vec![];
    for i in &untreated_list {
        let split = i.split("<");
        let mut splitted = split.map(String::from).collect::<Vec<String>>();
        if splitted.len() >= 2 {
            splitted.remove(0);
        }
        let joined: String = "<".to_string()+&splitted.join("<");

        //do it again
        let split = joined.split(">");
        let mut splitted = split.map(String::from).collect::<Vec<String>>();
        if splitted.len() >= 2 {
            splitted.remove(splitted.len()-1);
        }
        final_text.push(splitted.join(">")+">");
    }
    let joined = final_text.join("");
    dbg!(&joined);
    return joined;
}


//example input "<div>"
pub fn is_tag(text: String) -> (String, String) {
    let s = text.split("<");
    let mut split = s.map(String::from).collect::<Vec<String>>();
    split.remove(0); //left with "div>"
    let joined = split.join("");
    let s = joined.split(">");
    let mut split = s.map(String::from).collect::<Vec<String>>();
    split.remove(split.len()-1);
    let joined = split.join("");
    if joined.contains("/") {
        let joined= joined.replace("/", "");
        return (joined, "end".to_string());
    }
    return (joined, "start".to_string());
}

pub fn process_text(text: String) -> crate::TextFormat {
    let split = text.split("<");
    //at most a line of it would only have 3 items (assuming there's no "<" in the text itself)
    let mut vec = split.clone().map(String::from).collect::<SmallVec<[String; 3]>>();
    //the .clone() allocates memory for the clone but will be dropped after the line since it is not kept
    drop(split); //vec first clones split then splits it, since the original split is no longer used stays idle and we can drop it
    vec.remove(0);
    vec.remove(vec.len()-1);

    let joined = vec.join("<");
    drop(vec);

    let split = joined.split(">");
    let tag = split.clone().map(String::from).collect::<SmallVec<[String; 2]>>();
    drop(split);

    let mut vec_for_text = tag.clone();
    vec_for_text.remove(0);
    

    //some bullshit magic above please ignore
    let final_text = vec_for_text.join(">");
    drop(vec_for_text); //text<waow>
    
    let final_tag = tag[0].clone();
    //div class="test" link="github.com"
    return crate::TextFormat {
        tag: final_tag,
        text: final_text,
        class: "".to_string(),
        link: "".to_string(),
    };
}
