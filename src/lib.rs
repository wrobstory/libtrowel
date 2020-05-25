use std::collections::HashMap;
use std::error;
use std::fmt;

use scraper::{ElementRef, Html, Selector};

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

pub struct ColorTable {
    id_to_name: HashMap<i8, String>,
    name_to_id: HashMap<String, i8>,
}

pub struct Color {
    name: String,
    id: i8,
}

pub struct Part {
    known_colors: Vec<Color>,
}

pub fn parse_known_colors(part_color_page: &Html) -> Result<Vec<&str>, HtmlParsingError> {
    let td_selector =
        Selector::parse(r#"div[class="pciColorTitle"]"#).map_err(|_| HtmlParsingError::Select)?;
    let tds = part_color_page.select(&td_selector);
    let known_colors = match tds.last() {
        Some(td) => td.next_siblings(),
        None => return Err(HtmlParsingError::StructuralError),
    };
    Ok(known_colors
        .filter_map(|color_node| match ElementRef::wrap(color_node) {
            Some(element) => element.text().next(),
            None => None,
        })
        .collect::<Vec<&str>>())
}

pub fn parse_color_table(part_table_page: &Html) -> Result<ColorTable, HtmlParsingError> {
    let td_selector = Selector::parse(r#"table[id="id-main-legacy-table"]"#)
        .map_err(|_| HtmlParsingError::Select)?;
    Ok(ColorTable {
        id_to_name: HashMap::new(),
        name_to_id: HashMap::new(),
    })
}
