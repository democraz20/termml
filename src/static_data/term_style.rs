use ansi_term::Style;
use ansi_term::Colour;

fn get_color_from_string(text: &str) -> Colour {
    let text: &str = &text.to_lowercase();
    let text: &str = &text.replace(" ", "");
    match text {
        "black" => Colour::Black,
        "red" => Colour::Red,
        "green" => Colour::Green,
        "yellow" => Colour::Yellow,
        "blue" => Colour::Blue,
        "purple" => Colour::Purple,
        "cyan" => Colour::Cyan,
        "white" => Colour::White,
        _ => Colour::White
    }
}

fn get_rgb_from_string(text: &str) -> 
    Result<(u8, u8, u8), Box<dyn std::error::Error>> {
    let split = text.split(",");
    let mut vec = split.map(String::from).collect::<Vec<_>>();
    if !vec.len() == 3 {
        return Err(Box::from("RGB values count is not 3"))
    };
    for i in 0..3{
        vec[i] = vec[i].replace(" ", "");
    }
    let n1 = vec[0].parse::<u8>()?;
    let n2 = vec[0].parse::<u8>()?;
    let n3 = vec[0].parse::<u8>()?;
    Ok((n1, n2, n3))
}

//checking windows cmd.exe style support
//aka Windows Command Prompt
//[x] colors
//[x] underline
//[ ] bold
//[x] background color
//[x] RGB values (not tested on all terminals)
//[x] Fixed xterm-256 values (https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg)