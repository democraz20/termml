use ansi_term::Style;
use ansi_term::Colour;

fn get_color_from_string(text: &str) -> Colour {
    let text: &str = &text.to_lowercase();
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