use std::collections::HashMap;
use std::error;
use std::fmt;

use nipper::Document;

#[derive(Debug, Clone)]
pub enum HtmlParsingError {
    StructuralError,
    Select,
}

impl fmt::Display for HtmlParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HtmlParsingError::StructuralError => write!(
                f,
                "found unexpected HTML structure. Page HTML may have changed"
            ),
            HtmlParsingError::Select => write!(f, "error parsing css selector"),
        }
    }
}

impl error::Error for HtmlParsingError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct ColorGuide {
    id_to_name: HashMap<i8, String>,
    name_to_id: HashMap<String, i8>,
}

#[derive(Debug)]
pub struct Color {
    name: String,
    id: i8,
}

#[derive(Debug)]
pub struct Part {
    known_colors: Vec<Color>,
}

// TODO: Resultify this thing
pub fn parse_known_colors(part_color_page: &Document) -> Result<Vec<String>, HtmlParsingError> {
    let tdr = part_color_page
        .select("table.pciColorInfoTable")
        .select("tbody")
        .select("tr")
        .last()
        .select("span")
        .select("a")
        .iter()
        .map(|color| String::from(color.text()))
        .filter(|x| !x.starts_with("(Not Applicable)"))
        .collect::<Vec<String>>();
    Ok(tdr)
}
