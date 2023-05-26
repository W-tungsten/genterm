use std::{io, io::Write};

// Types of elements
#[derive(Clone)]
enum Element {
    SharpGtArrow(bool, u8, u8, u8, u8, u8, u8),
    SoftGtArrow(bool, u8, u8, u8, u8, u8, u8),
    SharpLtArrow(bool, u8, u8, u8, u8, u8, u8),
    SoftLtArrow(bool, u8, u8, u8, u8, u8, u8),
    CommandResult(bool, String, u8, u8, u8, u8, u8, u8),
    VariableValue(bool, String, u8, u8, u8, u8, u8, u8),
    Text(bool, String, u8, u8, u8, u8, u8, u8)
}

fn main() -> std::io::Result<()> {
    let elements: Vec<Element> = vec![
        // Change this bit
        // The first boolean is whether to ignore background colour
        // If it is false, just set the last 3 numbers to 0, they will be ignored
        // To save your changes, run cargo build -r -v in this directory then restart bash
        Element::CommandResult(false, String::from("pwd"), 2, 177, 252, 113, 255, 236),
        Element::Text(true, String::from(" "), 2, 177, 252, 0, 0, 0),
        Element::Text(false, String::from(" at "), 119, 89, 0, 252, 189, 2),
        Element::Text(false, String::from(" "), 71, 71, 71, 221, 221, 221),
        Element::CommandResult(true, String::from("date +%H:%M:%S"), 71, 71, 71, 0, 0, 0),
        Element::SharpGtArrow(true, 71, 71, 71, 0, 0, 0),
    ];


















































    print!("{}", render(elements)?);
    io::stdout().flush()?;
    Ok(())
}

impl Element {
    fn ignore_backgnd(self) -> bool {
        match self {
            Element::CommandResult(b, ..) => b,
            Element::SharpGtArrow(b, ..) => b,
            Element::SharpLtArrow(b, ..) => b,
            Element::SoftGtArrow(b, ..) => b,
            Element::SoftLtArrow(b, ..) => b,
            Element::Text(b, ..) => b,
            Element::VariableValue(b, ..) => b,
        }
    }
}

fn colour(include_backgnd: bool, element: &str, foreground: (u8, u8, u8), background: (u8, u8, u8),) -> String {
    // https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797 for help with colouring stuff in the terminal
    if include_backgnd {
        format!("\\x1b[38;2;{};{};{}m{}", foreground.0, foreground.1, foreground.2, element)
    } else {
        format!("\\x1b[38;2;{};{};{}m\\x1b[48;2;{};{};{}m{}", foreground.0, foreground.1, foreground.2, background.0, background.1, background.2, element)
    }
}

fn render(elements: Vec<Element>) -> std::io::Result<String> {
    let mut output = String::new();
    for element in elements {
        if element.clone().ignore_backgnd() == true {
            output = format!("{}{}", output, match element {
                Element::SharpGtArrow(_, a, b, c, _, _, _) => colour(true, ">", (a, b, c), (0, 0, 0)),
                Element::SoftGtArrow(_, a, b, c, _, _, _) => colour(true, ")", (a, b, c), (0, 0, 0)),
                Element::SharpLtArrow(_, a, b, c, _, _, _) => colour(true, "<", (a, b, c), (0, 0, 0)),
                Element::SoftLtArrow(_, a, b, c, _, _, _) => colour(true, "(", (a, b, c), (0, 0, 0)),
                Element::CommandResult(_, comm, a, b, c, _, _, _) => colour(true, &format!("$({})", comm), (a, b, c), (0, 0, 0)),
                Element::VariableValue(_, name, a, b, c, _, _, _) => colour(true, &format!("${}", name), (a, b, c), (0, 0, 0)),
                Element::Text(_, text, a, b, c, _, _, _) => colour(true, &text, (a, b, c), (0, 0, 0)),
            });
            continue;
        }
        output = format!("{}{}", output, match element {
            Element::SharpGtArrow(_, a, b, c, d, e, f) => colour(false, ">", (a, b, c), (d, e, f)),
            Element::SoftGtArrow(_, a, b, c, d, e, f) => colour(false, ")", (a, b, c), (d, e, f)),
            Element::SharpLtArrow(_, a, b, c, d, e, f) => colour(false, "<", (a, b, c), (d, e, f)),
            Element::SoftLtArrow(_, a, b, c, d, e, f) => colour(false, "(", (a, b, c), (d, e, f)),
            Element::CommandResult(_, comm, a, b, c, d, e, f) => colour(false, &format!("$({})", comm), (a, b, c), (d, e, f)),
            Element::VariableValue(_, name, a, b, c, d, e, f) => colour(false, &format!("${}", name), (a, b, c), (d, e, f)),
            Element::Text(_, text, a, b, c, d, e, f) => colour(false, &text, (a, b, c), (d, e, f)),
        });        
    }
    output += "\\x1b[0m";
    output += " ";
    Ok(output)
}


