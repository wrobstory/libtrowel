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
pub struct Color<'a> {
    name: String,
    id: Option<&'a i32>,
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

pub fn parse_known_colors<'a>(
    part_color_page: &Document,
    color_guide: &'a ColorGuide,
) -> Result<Vec<Color<'a>>, HtmlParsingError> {
    match select_color_anchors(part_color_page) {
        None => Err(HtmlParsingError),
        Some(a) => Ok(a
            .iter()
            .filter_map(|color| {
                let color_text = String::from(color.text());
                if color_text.starts_with("(Not Applicable)") {
                    None
                } else {
                    let color_id = color_guide.name_to_id.get(&color_text);
                    Some(Color {
                        name: color_text,
                        id: color_id.clone(),
                    })
                }
            })
            .collect::<Vec<Color>>()),
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

pub fn row_to_color<'a>(color_guide_row: &Selection) -> Option<(String, i32)> {
    let id_selection = select_color_id_from_row(color_guide_row)?;
    let id = id_selection
        .html()
        .replace("&nbsp;", "")
        .parse::<i32>()
        .ok()?;
    let color_name = select_color_name_from_row(color_guide_row)?
        .html()
        .replace("&nbsp;", "");
    Some((color_name, id))
}

pub fn parse_color_guide(color_guide_page: &Document) -> Result<ColorGuide, HtmlParsingError> {
    let mut id_to_name: HashMap<i32, String> = HashMap::new();
    let mut name_to_id: HashMap<String, i32> = HashMap::new();
    select_color_guide_rows(color_guide_page)
        .unwrap()
        .iter()
        .for_each(|tr| {
            let (color_name, color_id) = row_to_color(&tr).unwrap();
            id_to_name.insert(color_id, color_name.clone());
            name_to_id.insert(color_name, color_id);
        });
    Ok(ColorGuide {
        id_to_name: id_to_name,
        name_to_id: name_to_id,
    })
}
