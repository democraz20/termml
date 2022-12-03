use crate::static_data::structs::TermmlMain;


pub fn get(url: String) -> 
    Result<String, ureq::Error>
{
    let body: String = ureq::get(url.as_str())
        .call()?
        .into_string()?;
    Ok(body)
    // Ok(())
}
// pub fn get_from_url(url: String) -> 
//     Result<String, Box<dyn std::error::Error>> {
//     let set = ureq::get(url.as_str())
//         .set("Example-Header", "header value");
//     let call = match set.call() {
//         Ok(r) => r,
//         Err(e) => {
//             return Err(Box::new(e))
//         }
//     };
//     let into = match call.into_string() {
//         Ok(r) => r,
//         Err(e) => {
//             return Err(Box::new(e))
//         }
//     };
//     Ok(into)
// }

// pub fn retry(url: String) -> Result<String, ureq::Error> {
//     for i in 0..3 {
//         match get_from_url(url) {
//             Ok(r) => return Ok(r),
//             Err(e) => {}
//         }
//     }
//     return ureq::Error::Transport();
//     // Ok(())
// }