pub fn fetch(url: &String) -> Result<String, ureq::Error> {
    for i in 0..3 {
        println!("retrying attempt : {}", i + 1);
        match ureq::get(url.as_str()).call() {
            Ok(r) => match r.into_string() {
                Ok(r) => return Ok(r),
                Err(_) => {}
            },
            Err(ureq::Error::Status(code, response)) => {
                if i == 2 {
                    return Err(ureq::Error::Status(code, response));
                }
            }
            Err(ureq::Error::Transport(transport)) => {
                if i == 2 {
                    return Err(ureq::Error::Transport(transport));
                }
            }
        }
    }
    //retry once again, worst case scenario
    println!("retrying for the last time");
    let result = ureq::get(url.as_str()).call();
    let error = result.unwrap_err().into_transport().unwrap();

    Err(ureq::Error::Transport(error))
}
