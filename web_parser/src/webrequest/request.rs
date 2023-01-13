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
            } // #[allow(unreachable_patterns)]
              // Err(_) => {
              //     if i == 2  {
              //         return Err("Unknown error occured")?;
              //     }
              // }
        }
    }
    //retry once again, worst case scenario
    println!("retrying for the last time");
    let result = ureq::get(url.as_str()).call();
    let error = result.unwrap_err().into_transport().unwrap();

    Err(ureq::Error::Transport(error))
    // Ok(())
}

pub fn get_filename(url: &String) -> String {
    let mut vec = url.split("/").map(String::from).collect::<Vec<_>>();
    if vec.len() <= 3 {
        return String::new();
    }
    if vec.len() >= 4 {
        for _ in 0..3 {
            vec.remove(0);
        }
    }
    vec.join("/")
}
