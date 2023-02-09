use std::fs;
use std::path::Path;

pub struct Logger {
    contents: Vec<String>,
    log_title: String,
    save_path: String,
    clear_init: bool, //
    re_save: bool,
}

impl Logger {
    pub fn new(title: &str, path: &str, clearinit: bool, resave: bool) -> Logger {
        //clean text input
        let mut title = title.replace("\n", "");
        title = title.replace("\r", "");

        Logger {
            contents: vec![format!("[[{}]]", title.to_string())],
            log_title: title.to_string(),
            save_path: path.to_string(),
            clear_init: clearinit,
            re_save: resave,
        }
    }
    pub fn save(&mut self) -> Result<(), std::io::Error> {
        let content = self.contents.join("\n");
        match self.clear_init {
            true => {
                //creates new file anyways
                fs::File::create(&self.save_path)?;
            }
            false => {
                //somehow keep the log
            }
        }
        match self.re_save {
            true => {
                if Path::new(&self.save_path).exists() {
                    fs::write(self.save_path.to_string(), content)?;
                    //fs::write writes the entire content of the file
                    //clear the log buffer
                    self.contents = vec![format!("[[{}]]", self.log_title)];
                } else {
                    fs::File::create(&self.save_path)?;
                    fs::write(self.save_path.to_string(), content)?;
                    self.contents = vec![format!("[[{}]]", self.log_title)];
                }
            }
            false => {
                //no re-saving,
                if Path::new(&self.save_path).exists() {
                    fs::write(self.save_path.to_string(), content)?;
                    //fs::write writes the entire content of the file
                } else {
                    fs::File::create(&self.save_path)?;
                    fs::write(self.save_path.to_string(), content)?;
                }
            }
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
