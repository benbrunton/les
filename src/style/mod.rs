
use ansi_term::{Colour, Style};

const ICON_UNKNOWN: char = '\u{2002}';

#[derive(Debug, PartialEq, Clone)]
pub struct PaintItem{
    pub label: String,
    pub is_bold: bool,
    pub is_underline: bool,
    pub is_dimmed: bool,
    pub is_hidden: bool,
    pub colour: Option<Colour>,
    pub icon: Option<char>
}

pub fn paint(item: &PaintItem) -> String {

    let label = get_decorated_text(item);
    let icon = match item.icon {
        Some(i) => i.to_string(),
        __      => format!("{} ", ICON_UNKNOWN)
    };

    return format!(
        "{} {}",
        Style::new().bold().paint(icon),
        label
    );
}


fn get_decorated_text(item: &PaintItem) -> String {

    let mut style = Style::new();

    if item.is_bold {
        style = style.bold();
    }

    if item.is_underline {
        style = style.underline();
    }

    if item.is_dimmed {
        style = style.dimmed();
    }

    if item.is_hidden {
        style = style.hidden();
    }

    if let Some(colour) = item.colour {
        style = style.fg(colour);
    }

    style.paint(item.label.clone()).to_string()
}

#[cfg(test)]
mod tests {

    use style::*;

    #[test]
    fn it_paints_colours() {
        let licenseItem = PaintItem{
            label: "LICENSE".to_string(),
            is_bold: false,
            is_underline: false,
            is_dimmed: false,
            is_hidden: false,
            colour: Some(Colour::White),
            icon: None
        };

        let actual = paint(&licenseItem);
        assert_eq!(actual, "\u{1b}[1m\u{2002} \u{1b}[0m \u{1b}[37mLICENSE\u{1b}[0m");
    }

    #[test]
    fn it_paints_directory_icons() {
        let licenseItem = PaintItem{
            label: "scripts/".to_string(),
            is_bold: false,
            is_underline: false,
            is_dimmed: false,
            is_hidden: false,
            colour: None,
            icon: Some('\u{1F4C2}')
        };

        let actual = paint(&licenseItem);
        assert_eq!(actual, "\u{1b}[1m\u{1F4C2}\u{1b}[0m scripts/");

    }

}
