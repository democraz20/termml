use ureq::Response;

use crate::static_data::structs::TermmlMain;
use hard_xml::XmlWrite;

pub fn get(url: String) -> 
    // Result<String, ureq::Error>
    Result<String, Box<dyn std::error::Error>>
{
    for i in 0..3 {
        println!("retrying attemp : {}", i);
        match ureq::get(url.as_str()).call() {
            Ok(r) => {
                match r.into_string() {
                    Ok(r) => {
                        return Ok(r)
                    },
                    Err(_) => {}
                }
            },
            Err(ureq::Error::Status(code, response)) => {
                if i == 2 {
                    return Err(Box::new(ureq::Error::Status(code, response)))
                }
            },
            Err(ureq::Error::Transport(transport)) => {
                if i == 2 {
                    return Err(Box::new(ureq::Error::Transport(transport)))
                }
            },
            #[allow(unreachable_patterns)]
            Err(_) => {
                if i == 2  {
                    return Err("Unknown error occured")?;
                }
            }
        }
    }
    // Err(ureq::Error::Status(0, ()))
    Err("Unknown error occured")?
    // Ok(())
}