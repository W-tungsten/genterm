// Reforged (1)

// This will fairly soon have YAML parsing for proper configuration.
// Using this terminal prompt (at least for now) pretty much _requires_ you to have the following:
// A Nerd Font
// 24-bit color terminal support
// main_old.rs here is deprecated and also just terrible code

// Do not do any of the following:
// Distribute this code verbatim or >= 95% verbatim without a link to the GitHub at https://github.com/W-tungsten/genterm
// (Or a similar link taking you to the GitHub source)
// Claim that the code verbatim or >= 95% verbatim is yours
// Distribute this code verbatim or >= 95% verbatim without the comments from the line saying
// "Do not do any of the following:" onwards.
// Otherwise, anyone can do anything they like with this code.
// (This also applies to main_old.rs)

static ARROW_TYPES: &[ElementType] = &[
    ElementType::SharpGtArrow,
    ElementType::SharpLtArrow,
    ElementType::SoftGtArrow,
    ElementType::SoftLtArrow,
];

use std::{fs::File, io, io::Read, io::Write};

#[derive(Copy, Clone, PartialEq)]
enum ElementType {
    SharpGtArrow,
    SharpLtArrow,
    SoftGtArrow,
    SoftLtArrow,
    CommandResult,
    VariableValue,
    Text,
}

#[derive(Clone)]
enum ElementContents {
    StringColors(String, Option<Color>, Option<Color>),
    Colors(Option<Color>, Option<Color>),
}

#[derive(Clone)]
struct Element {
    variant: ElementType,
    contents: ElementContents,
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

trait Render {
    fn render(self) -> String;
}

trait Correct {
    fn correct(self) -> Element;
}

impl Correct for Vec<Element> {
    fn correct(self) -> Element {
        /* *** PLEASE NOTE THAT THIS FUNCTION ASSUMES THAT THE FIRST ELEMENT IS AN ARROW.
        IF IT ISN'T THIS FUNCTION IS UNNECESSARY. *** */
        Element {
            variant: self[0].variant,
            contents: match (self[0].contents.clone(), self[1].contents.clone()) {
                (ElementContents::Colors(_, fg), ElementContents::Colors(_, bg))
                | (ElementContents::Colors(_, fg), ElementContents::StringColors(_, bg, _)) => {
                    ElementContents::Colors(fg, bg)
                }
                _ => unimplemented!(),
            },
        }
    }
}

impl Render for Vec<Element> {
    fn render(self) -> String {
        let mut output = String::new();

        for window in self.windows(2) {
            output += &if ARROW_TYPES.contains(&window[0].variant) {
                    window.to_vec().correct()
                } else {
                    window[0].clone()
                }
                .render();
        }

        if let Some(last) = self.last() {
            output += &if ARROW_TYPES.contains(&last.variant) {
                vec![last.clone(), Element {
                    variant: ElementType::Text,
                    contents: ElementContents::StringColors("".to_owned(), None, None),
                }].correct()
            } else {
                last.clone()
            }
                .render();
        }



        if !output.is_empty() {
            output + r"\x01\e[0m\x02"
        } else {
            output
        }
    }
}

impl Render for Element {
    fn render(self) -> String {
        let (variant, contents) = (self.variant, self.contents);
        if [
            ElementType::SharpGtArrow,
            ElementType::SharpLtArrow,
            ElementType::SoftGtArrow,
            ElementType::SharpLtArrow,
        ]
        .contains(&variant)
        {
            let (fg, bg) = match contents {
                ElementContents::Colors(fg, bg) => (fg, bg),
                _ => unreachable!(),
            };
            (fg, bg).sequence()
                + match variant {
                    ElementType::SharpGtArrow => "",
                    ElementType::SharpLtArrow => "",
                    ElementType::SoftGtArrow => "",
                    ElementType::SoftLtArrow => "", //      ▓▒░    ░▒▓ 
                    _ => unreachable!(),
                }
        } else {
            let (str, fg, bg) = match contents {
                ElementContents::StringColors(str, fg, bg) => (str, fg, bg),
                _ => unreachable!(),
            };
            (fg, bg).sequence()
                + &match variant {
                    ElementType::CommandResult => format!(r"\x01$({})\x02", str),
                    ElementType::Text => str,
                    ElementType::VariableValue => format!("${}", str),
                    _ => unreachable!(),
                }
        }
    }
}

trait Sequence {
    fn sequence(self) -> String;
}

impl Sequence for (Option<Color>, Option<Color>) {
    fn sequence(self) -> String {
        let mut output = String::new();

        if let Some(fg) = self.0 {
            output += &format!(r"\x1b[38;2;{};{};{}m", fg.r, fg.g, fg.b);
        }

        if let Some(bg) = self.1 {
            output += &format!(r"\x1b[48;2;{};{};{}m", bg.r, bg.g, bg.b);
        }

        if &output != "" {
            r"\x01".to_owned() + &output + r"\x02"
        } else {
            output
        }
    }
}

fn main() -> io::Result<()> {
    /*let default_config = "".as_bytes();
    let mut file_contents = String::new();
    if let Ok(mut file) = File::open("~/.genterm/config.toml") {
        file.read_to_string(&mut file_contents).unwrap_or_else(|_| {
            eprintln!(
                "Failed to read config file ...
Attempting to create config file with default values ..."
            );
            0
        });
    } else {
        if let Ok(mut file) = File::create("~/.genterm/config.txt") {
            file.write_all(default_config)
                .expect("Failed to make config file");
        }
    }*/

    let v = vec![
        Element {
            variant: ElementType::CommandResult,
            contents: ElementContents::StringColors(
            "pwd".to_owned(),
             Some(Color { r: 0, g: 255, b: 0 }),
              Some(Color { r: 255, g: 0, b: 255 }))
        },
        Element {
            variant: ElementType::SharpGtArrow,
            contents: ElementContents::Colors(None, None),
        }
    ];

    println!("{}", v.render());
    Ok(())
}
