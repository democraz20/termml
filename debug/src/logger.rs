use std::fs;
use std::path::Path;

pub struct Logger {
    contents: Vec<String>,
    log_title: String,
    save_path: String,
    clear_init: bool, //clears the file upon init
}

impl Logger {
    pub fn new(title: &str, path: &str, clearinit: bool) -> Logger {
        //clean text input
        let mut title = title.replace("\n", "");
        title = title.replace("\r", "");

        match clearinit {
            true => {
                return Logger {
                    contents: vec![format!("[[{}]]", title.to_string())],
                    log_title: title.to_string(),
                    save_path: path.to_string(),
                    clear_init: clearinit,
                }
            }
            false => { //dont clear the log on init
                let old_content: Vec<String> = match fs::read_to_string(path) {
                    Err(_) => Vec::new(),
                    Ok(r) => {
                        //parse old content
                        //remove the header
                        let mut v = r.split("\n").map(String::from).collect::<Vec<String>>();
                        v.pop();
                        v
                    }
                };
                let mut c = vec![format!("[[{}]]", title.to_string())];
                c.extend(old_content);
                return Logger {
                    contents: c,
                    log_title: title.to_string(),
                    save_path: path.to_string(),
                    clear_init: clearinit,
                }
            }
        }

    }
    pub fn save(&mut self) -> Result<(), std::io::Error> {
        let content = self.contents.join("\n");
        if Path::new(&self.save_path).exists() {
            fs::write(self.save_path.to_string(), content)?;
        } else {
            fs::File::create(self.save_path.to_string())?;
            fs::write(self.save_path.to_string(), content)?;
        }
        Ok(())
    }
    pub fn add(&mut self, content: &str) {
        //clean the content
        let mut content = content.replace("\n", "");
        content = content.replace("\r", "");
        //verify
        if self.contents.len() == 0 {
            self.contents.push(format!("[[{}]]", self.log_title));
            self.contents.push(content.to_string());
        }
        //more than 0 //cannot go below 0
        else {
            self.contents.push(content.to_string());
        }
    }
}
