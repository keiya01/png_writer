use crate::style::Style;
use crate::Error;
use rusttype::Font;
use std::{ops::Range, str};

#[derive(Debug)]
pub(crate) struct SplitText {
    pub(crate) text: String,
    pub(crate) style: Option<Style>,
    pub(crate) font: Option<Font<'static>>,
    pub(crate) range: Range<usize>,
}

/// TextArea is box to store each text with style.
/// For example you can set style to text one by one.
#[derive(Debug, Default)]
pub struct TextArea(pub(super) Vec<SplitText>);

impl TextArea {
    pub fn new() -> TextArea {
        TextArea(vec![])
    }

    /// Push text with style.
    pub fn push(&mut self, text: &str, style: Style, font: Option<Vec<u8>>) -> Result<(), Error> {
        let last_range_end = match self.0.iter().last() {
            Some(split) => split.range.end,
            None => 0,
        };

        let font = match font {
            Some(font) => Some(match Font::try_from_vec(font) {
                Some(font) => font,
                None => return Err(Error::InvalidFontBytes),
            }),
            None => None,
        };

        let mut string = String::new();
        string.push_str(text);

        let split_text = SplitText {
            text: string,
            style: Some(style),
            font,
            range: last_range_end..last_range_end + text.len(),
        };

        self.0.push(split_text);

        Ok(())
    }

    /// Push text without style.
    /// Style is override with parent style.
    /// Parent style is set with [`OGImageWriter::set_textarea()`](crate::writer::OGImageWriter::set_textarea).
    pub fn push_text(&mut self, text: &str) {
        let last_range_end = match self.0.iter().last() {
            Some(split) => split.range.end,
            None => 0,
        };

        let mut string = String::new();
        string.push_str(text);

        let split_text = SplitText {
            text: string,
            style: None,
            font: None,
            range: last_range_end..last_range_end + text.len(),
        };

        self.0.push(split_text);
    }

    pub(super) fn as_string(&self) -> String {
        let mut text = String::new();
        for split_text in &self.0 {
            text.push_str(&split_text.text);
        }
        text
    }

    pub(crate) fn get_split_text_from_char_range(&self, range: Range<usize>) -> Option<&SplitText> {
        for split_text in &self.0 {
            if split_text.range.start <= range.start && range.end <= split_text.range.end {
                return Some(split_text);
            }
        }
        None
    }
}
