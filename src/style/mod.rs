use ansi_term::{Colour, Style};
use fs::File;
use fs::DirType;

const ICON_DIRECTORY: char = '\u{1F4C2}';
const ICON_FILE: char = '\u{1F5CB}';
const ICON_UNKNOWN: char = '*';

pub fn paint(path: File) -> String {
    let icon = get_icon(path.get_dir_type());
    let label = get_decorated_text(path.get_dir_type(), path.to_str());

    return format!(
        "{} {}",
        Style::new().bold().paint(icon.to_string()),
        label
    );
}

fn get_icon(inode_type: DirType) -> char {
    match inode_type {
        DirType::Dir  => ICON_DIRECTORY,
        DirType::File => ICON_FILE,
        _             => ICON_UNKNOWN,
    }
}

fn get_decorated_text(inode_type: DirType, label: String) -> String {

    match inode_type {
        DirType::Dir => format!("{}", Style::new().fg(Colour::White).paint(label)),
        _            => label
    }
}

#[cfg(test)]
mod tests {

    use style::*;
    use ansi_term::Style;
    use fs;

    #[test]
    fn it_paints_file_icons() {
        let label = "LICENSE".to_string();
        let dir_type = fs::DirType::File;
        let f = File::new(label, dir_type);
        let actual = paint(f);
        assert_eq!(actual, "\u{1b}[1m🗋\u{1b}[0m LICENSE");
    }

    #[test]
    fn it_paints_directory_icons() {
        let label = "scripts/".to_string();
        let dir_type = fs::DirType::Dir;
        let f = File::new(label, dir_type);
        let actual = paint(f);
        assert_eq!(actual, "\u{1b}[1m📂\u{1b}[0m \u{1b}[37mscripts/\u{1b}[0m");
    }
}
