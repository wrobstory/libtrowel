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
    id_to_name: HashMap<i32, String>,
    name_to_id: HashMap<String, i32>,
}

#[derive(Debug)]
pub struct Color {
    name: String,
    id: i32,
}

#[derive(Debug)]
pub struct Part {
    known_colors: Vec<Color>,
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
}

pub fn select_color_id_from_row<'a>(color_guide_row: &'a Selection) -> Option<Selection<'a>> {
    color_guide_row
        .try_select(":first-child")?
        .try_select(":first-child")?
        .try_select(":first-child")
}

pub fn select_color_name_from_row<'a>(color_guide_row: &'a Selection) -> Option<Selection<'a>> {
    color_guide_row
        .try_select(":nth-child(n + 4)")?
        .try_select(":first-child")?
        .try_select(":first-child")
}

pub fn row_to_color(color_guide_row: &Selection) -> Option<Color> {
    let id_selection = select_color_id_from_row(color_guide_row)?;
    let id = id_selection
        .html()
        .replace("&nbsp;", "")
        .parse::<i32>()
        .ok()?;
    let color_name = select_color_name_from_row(color_guide_row)?
        .html()
        .replace("&nbsp;", "");
    Some(Color {
        id: id,
        name: color_name,
    })
}

pub fn parse_color_guide(color_guide_page: &Document) -> Result<ColorGuide, HtmlParsingError> {
    let mut id_to_name: HashMap<i32, String> = HashMap::new();
    let mut name_to_id: HashMap<String, i32> = HashMap::new();
    select_color_guide_rows(color_guide_page)
        .unwrap()
        .iter()
        .for_each(|tr| {
            let color = row_to_color(&tr).unwrap();
            id_to_name.insert(color.id, color.name.clone());
            name_to_id.insert(color.name, color.id);
        });
    Ok(ColorGuide {
        id_to_name: id_to_name,
        name_to_id: name_to_id,
    })
}
