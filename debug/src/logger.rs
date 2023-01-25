use std::path::Path;
use std::fs;

pub struct Logger {
    contents: Vec<String>,
    log_title: String,
    save_path: String,
    re_save: bool
}

impl Logger {
    pub fn new(&self, title: &str, path: &str, resave: bool) -> Logger {
        Logger {
            contents: vec![self.log_title.to_string()],
            log_title: title.to_string(),
            save_path: path.to_string(),
            re_save: resave
        }
    }
    pub fn save(&mut self) -> Result<(), std::io::Error> {
        let content = self.contents.join("\n");
        match self.re_save {
            true => {
                if Path::new(&self.save_path).exists() {
                    fs::remove_file(self.save_path.to_string())?;
                    fs::File::create(self.save_path.to_string())?;
                    fs::write(self.save_path.to_string(), content)?;
                    //fs::write writes the entire content of the file
                    //clear the log buffer
                    self.contents = vec![self.log_title.to_string()]
                }
            }
            false => {
                //no re-saving,
                if Path::new(&self.save_path).exists() {
                    fs::write(self.save_path.to_string(), content)?;
                    //fs::write writes the entire content of the file
                }
            }
        }
        Ok(())
    }
}