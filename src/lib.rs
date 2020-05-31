use std::collections::HashMap;
use std::error;
use std::fmt;

use nipper::{Document, Selection};

#[derive(Debug, Clone)]
pub struct HtmlParsingError;

impl fmt::Display for HtmlParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "found unexpected HTML structure. Page HTML may have changed"
        )
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
pub struct Color<'a> {
    name: &'a str,
    id: i8,
}

#[derive(Debug)]
pub struct Part<'a> {
    known_colors: Vec<Color<'a>>,
}

pub fn select_color_anchors(part_color_page: &Document) -> Option<Selection> {
    part_color_page
        .try_select("table.pciColorInfoTable")?
        .try_select("tbody")?
        .try_select("tr")?
        .last()
        .try_select("span")?
        .try_select("a")
}


pub fn parse_known_colors(part_color_page: &Document) -> Result<Vec<String>, HtmlParsingError> {
    match select_color_anchors(part_color_page) {
        None => Err(HtmlParsingError),
        Some(a) => Ok(a
            .iter()
            .map(|color| String::from(color.text()))
            .filter(|x| !x.starts_with("(Not Applicable)"))
            .collect::<Vec<String>>()),
    }
}

pub fn select_color_guide_rows(color_guide_page: &Document) -> Option<Selection> {
    color_guide_page
        .try_select(r#"table[id="id-main-legacy-table"]"#)?
        .try_select(r#"table[border="0"][cellpadding="1"][cellspacing="0"]"#)?
        .try_select("tr:nth-child(n + 2)")
        // .try_select(r#":not([height="20"])"#)
}

pub fn select_color_id_from_row<'a>(color_guide_row: &'a Selection) -> Option<Selection<'a>> {
    color_guide_row
        .try_select("td")?
        .try_select(":first-child")
}

pub fn parse_color_guide(color_guide_page: &Document) -> Result<(), HtmlParsingError> {
    select_color_guide_rows(color_guide_page).unwrap()
        .iter()
        .for_each(|x| {
            println!("{:?}", x.html());
        });
    Ok(())
}
