use crate::style::*;
use crate::Style;

pub struct NumberConfig {
    pub number: Style,
}

impl Default for NumberConfig {
    fn default() -> Self {
        NumberConfig { number: cyan() }
    }
}

pub struct UuidConfig {
    pub number: Style,
    pub letter: Style,
    pub dash: Style,
}

impl Default for UuidConfig {
    fn default() -> Self {
        UuidConfig {
            number: blue_and_italic(),
            letter: magenta_and_italic(),
            dash: red(),
        }
    }
}

pub struct UnixPathConfig {
    pub segment: Style,
    pub separator: Style,
}

impl Default for UnixPathConfig {
    fn default() -> Self {
        UnixPathConfig {
            segment: green(),
            separator: yellow(),
        }
    }
}

pub struct QuoteConfig {
    pub quotes_token: char,
    pub color: Style,
}

impl Default for QuoteConfig {
    fn default() -> Self {
        QuoteConfig {
            quotes_token: '"',
            color: yellow(),
        }
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct KeywordConfig {
    pub words: Vec<String>,
    pub style: Style,
}
