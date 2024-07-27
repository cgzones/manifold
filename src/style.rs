use nu_ansi_term::{Color as NuColor, Style as NuStyle};

pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub faint: bool,
    pub italic: bool,
    pub underline: bool,
}

pub enum Color {
    Black,
    DarkGray,
    Red,
    LightRed,
    Green,
    LightGreen,
    Yellow,
    LightYellow,
    Blue,
    LightBlue,
    Purple,
    LightPurple,
    Magenta,
    LightMagenta,
    Cyan,
    LightCyan,
    White,
    LightGray,
    Default,
}

impl From<&Color> for NuColor {
    fn from(color: &Color) -> Self {
        match color {
            Color::Black => NuColor::Black,
            Color::DarkGray => NuColor::DarkGray,
            Color::Red => NuColor::Red,
            Color::LightRed => NuColor::LightRed,
            Color::Green => NuColor::Green,
            Color::LightGreen => NuColor::LightGreen,
            Color::Yellow => NuColor::Yellow,
            Color::LightYellow => NuColor::LightYellow,
            Color::Blue => NuColor::Blue,
            Color::LightBlue => NuColor::LightBlue,
            Color::Purple => NuColor::Purple,
            Color::LightPurple => NuColor::LightPurple,
            Color::Magenta => NuColor::Magenta,
            Color::LightMagenta => NuColor::LightMagenta,
            Color::Cyan => NuColor::Cyan,
            Color::LightCyan => NuColor::LightCyan,
            Color::White => NuColor::White,
            Color::LightGray => NuColor::LightGray,
            Color::Default => NuColor::Default,
        }
    }
}

impl From<Style> for NuStyle {
    fn from(style: Style) -> Self {
        let mut nu_style = NuStyle::new();

        if let Some(fg) = &style.fg {
            nu_style = nu_style.fg(NuColor::from(fg));
        }
        if let Some(bg) = &style.bg {
            nu_style = nu_style.on(NuColor::from(bg));
        }
        if style.bold {
            nu_style = nu_style.bold();
        }
        if style.faint {
            nu_style = nu_style.dimmed();
        }
        if style.italic {
            nu_style = nu_style.italic();
        }
        if style.underline {
            nu_style = nu_style.underline();
        }

        nu_style
    }
}