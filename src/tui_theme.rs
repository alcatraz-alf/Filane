use ratatui::style::Color;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TuiTheme {
    Dark,
    Dracula,
    Nord,
    Monokai,
    SolarizedDark,
}

/// Colors for every painted TUI surface. In practice `panel_bg` paints
/// over the terminal's own background so the app never inherits a white
/// or oddly themed terminal setting (GitHub issue #1).
#[derive(Debug, Clone, Copy)]
pub struct TuiPalette {
    pub panel_bg: Color,
    pub text: Color,
    pub text_dim: Color,
    pub border_active: Color,
    pub border_dim: Color,
    pub sel_bg: Color,
    pub sel_fg: Color,
    pub multi_bg: Color,
    pub multi_fg: Color,
    pub title: Color,
    pub status: Color,
}

impl TuiTheme {
    pub fn all() -> [TuiTheme; 5] {
        [
            TuiTheme::Dark,
            TuiTheme::Dracula,
            TuiTheme::Nord,
            TuiTheme::Monokai,
            TuiTheme::SolarizedDark,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            TuiTheme::Dark => "Dark",
            TuiTheme::Dracula => "Dracula",
            TuiTheme::Nord => "Nord",
            TuiTheme::Monokai => "Monokai",
            TuiTheme::SolarizedDark => "Solarized Dark",
        }
    }

    pub fn next(&self) -> Self {
        let list = Self::all();
        let idx = list.iter().position(|t| t == self).unwrap_or(0);
        list[(idx + 1) % list.len()]
    }

    pub fn palette(&self) -> TuiPalette {
        match self {
            TuiTheme::Dark => TuiPalette {
                panel_bg: Color::Rgb(45, 47, 50),
                text: Color::Rgb(232, 234, 237),
                text_dim: Color::Rgb(154, 160, 166),
                border_active: Color::Rgb(66, 133, 244),
                border_dim: Color::Rgb(105, 110, 118),
                sel_bg: Color::Rgb(66, 133, 244),
                sel_fg: Color::White,
                multi_bg: Color::Rgb(66, 70, 77),
                multi_fg: Color::Rgb(232, 234, 237),
                title: Color::Rgb(138, 180, 248),
                status: Color::Rgb(251, 188, 5),
            },
            TuiTheme::Dracula => TuiPalette {
                panel_bg: Color::Rgb(40, 42, 54),
                text: Color::Rgb(248, 248, 242),
                text_dim: Color::Rgb(98, 114, 164),
                border_active: Color::Rgb(189, 147, 249),
                border_dim: Color::Rgb(68, 71, 90),
                sel_bg: Color::Rgb(79, 82, 103),
                sel_fg: Color::Rgb(255, 121, 198),
                multi_bg: Color::Rgb(68, 71, 90),
                multi_fg: Color::Rgb(139, 233, 253),
                title: Color::Rgb(189, 147, 249),
                status: Color::Rgb(241, 250, 140),
            },
            TuiTheme::Nord => TuiPalette {
                panel_bg: Color::Rgb(46, 52, 64),
                text: Color::Rgb(236, 239, 244),
                text_dim: Color::Rgb(143, 158, 178),
                border_active: Color::Rgb(136, 192, 208),
                border_dim: Color::Rgb(76, 86, 106),
                sel_bg: Color::Rgb(67, 76, 94),
                sel_fg: Color::Rgb(136, 192, 208),
                multi_bg: Color::Rgb(59, 66, 82),
                multi_fg: Color::Rgb(163, 190, 140),
                title: Color::Rgb(136, 192, 208),
                status: Color::Rgb(235, 203, 139),
            },
            TuiTheme::Monokai => TuiPalette {
                panel_bg: Color::Rgb(39, 40, 34),
                text: Color::Rgb(248, 248, 242),
                text_dim: Color::Rgb(117, 113, 94),
                border_active: Color::Rgb(166, 226, 46),
                border_dim: Color::Rgb(90, 88, 74),
                sel_bg: Color::Rgb(249, 38, 114),
                sel_fg: Color::White,
                multi_bg: Color::Rgb(73, 72, 62),
                multi_fg: Color::Rgb(230, 219, 116),
                title: Color::Rgb(166, 226, 46),
                status: Color::Rgb(230, 219, 116),
            },
            TuiTheme::SolarizedDark => TuiPalette {
                panel_bg: Color::Rgb(7, 54, 66),
                text: Color::Rgb(238, 232, 213),
                text_dim: Color::Rgb(131, 148, 150),
                border_active: Color::Rgb(38, 139, 210),
                border_dim: Color::Rgb(88, 110, 117),
                sel_bg: Color::Rgb(38, 139, 210),
                sel_fg: Color::Rgb(253, 246, 227),
                multi_bg: Color::Rgb(0, 43, 54),
                multi_fg: Color::Rgb(181, 137, 0),
                title: Color::Rgb(38, 139, 210),
                status: Color::Rgb(181, 137, 0),
            },
        }
    }
}

fn config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config/filane")
}

/// Read the persisted theme name (defaults to Dark when absent/corrupt).
pub fn load_tui_theme() -> TuiTheme {
    let path = config_path().join("tui-theme");
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .and_then(|name| {
            TuiTheme::all()
                .into_iter()
                .find(|t| t.name() == name)
        })
        .unwrap_or(TuiTheme::Dark)
}

/// Persist the theme selection to ~/.config/filane/tui-theme.
pub fn save_tui_theme(theme: TuiTheme) {
    let dir = config_path();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("tui-theme"), theme.name());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycling_wraps_through_all_themes() {
        let mut t = TuiTheme::Dark;
        let mut names = vec![];
        for _ in 0..6 {
            names.push(t.name());
            t = t.next();
        }
        let all: Vec<&str> = TuiTheme::all().iter().map(|t| t.name()).collect();
        assert_eq!(names.len(), 6);
        assert_eq!(names[5], "Dark");
        for n in &all {
            assert!(names.contains(n));
        }
    }

    #[test]
    fn persistence_roundtrip() {
        save_tui_theme(TuiTheme::Nord);
        assert_eq!(load_tui_theme(), TuiTheme::Nord);
        save_tui_theme(TuiTheme::Dark);
        assert_eq!(load_tui_theme(), TuiTheme::Dark);
    }
}
