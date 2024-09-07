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

pub struct KeyValueConfig {
    pub key: Style,
    pub separator: Style,
}

impl Default for KeyValueConfig {
    fn default() -> Self {
        KeyValueConfig {
            key: faint(),
            separator: white(),
        }
    }
}

pub struct TimeConfig {
    pub time: Style,
    pub zone: Style,
    pub separator: Style,
}

impl Default for TimeConfig {
    fn default() -> Self {
        TimeConfig {
            time: blue(),
            zone: red(),
            separator: faint(),
        }
    }
}

pub struct IpV4Config {
    pub number: Style,
    pub separator: Style,
}

impl Default for IpV4Config {
    fn default() -> Self {
        IpV4Config {
            number: blue_and_italic(),
            separator: red(),
        }
    }
}

pub struct IpV6Config {
    pub number: Style,
    pub letter: Style,
    pub separator: Style,
}

impl Default for IpV6Config {
    fn default() -> Self {
        IpV6Config {
            number: blue_and_italic(),
            letter: magenta_and_italic(),
            separator: red(),
        }
    }
}

pub struct UrlConfig {
    pub http: Style,
    pub https: Style,
    pub host: Style,
    pub path: Style,
    pub query_params_key: Style,
    pub query_params_value: Style,
    pub symbols: Style,
}

impl Default for UrlConfig {
    fn default() -> Self {
        UrlConfig {
            http: red_and_faint(),
            https: green_and_faint(),
            host: blue_and_faint(),
            path: blue(),
            query_params_key: magenta(),
            query_params_value: cyan(),
            symbols: red(),
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

pub struct UnixProcessConfig {
    pub name: Style,
    pub id: Style,
    pub bracket: Style,
}

impl Default for UnixProcessConfig {
    fn default() -> Self {
        UnixProcessConfig {
            name: green(),
            id: yellow(),
            bracket: red(),
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
