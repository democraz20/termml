use ansi_term::Style;
use ansi_term::Colour;

fn get_color_from_string(text: &str) -> Colour {
    let text: &str = &text.to_lowercase();
    let text: &str = &text.replace(" ", "");
    let text: &str = &text.replace("\n", "");
    let text: &str = &text.replace("\r", "");
    match text {
        "black" => Colour::Black,
        "red" => Colour::Red,
        "green" => Colour::Green,
        "yellow" => Colour::Yellow,
        "blue" => Colour::Blue,
        "purple" => Colour::Purple,
        "cyan" => Colour::Cyan,
        "white" => Colour::White,
        _ => 
        {
            if text.contains(",") {
                get_color_from_string(text)
            }
            else {
                Colour::White
            }
        }
    }
}

fn get_rgb_from_string(text: &str) -> 
    Result<(u8, u8, u8), Box<dyn std::error::Error>> {
    let split = text.split(",");
    let mut vec = split.map(String::from).collect::<Vec<_>>();
    if !vec.len() == 3 {
        return Err(Box::from("RGB values count is not 3"))
    };

    //not sure about this since the string should have 
    //been trimmed according to get_color_from_string
    for i in 0..3{
        vec[i] = vec[i].replace(" ", "");
    }
    let n1 = vec[0].parse::<u8>()?;
    let n2 = vec[0].parse::<u8>()?;
    let n3 = vec[0].parse::<u8>()?;
    Ok((n1, n2, n3))
}

fn get_fixed_from_string(text: &str) -> 
    Result<u8, Box<dyn std::error::Error>> {
    Ok(u8::MAX)
    //u8 max is 255 
    //xterm-256-color chart actually has 255 colors
    //weird naming convention
}

//checking windows cmd.exe style support
//aka Windows Command Prompt
//[x] colors
//[x] underline
//[ ] bold
//[x] background color
//[x] RGB values (not tested on all terminals)
//[x] Fixed xterm-256 values (https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg)