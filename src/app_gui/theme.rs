use super::{FileManagerApp, Theme};
use egui;

impl FileManagerApp {
    pub(crate) fn get_modern_visuals(&self) -> egui::Visuals {
        match self.current_theme {
            Theme::Dark => self.get_dark_theme(),
            Theme::Light => self.get_light_theme(),
            Theme::Dracula => self.get_dracula_theme(),
            Theme::Nord => self.get_nord_theme(),
            Theme::Monokai => self.get_monokai_theme(),
            Theme::SolarizedDark => self.get_solarized_dark_theme(),
        }
    }

    pub(crate) fn get_dark_theme(&self) -> egui::Visuals {
        let mut visuals = egui::Visuals::dark();

        visuals.window_rounding = 8.0.into();
        visuals.window_shadow = egui::epaint::Shadow {
            offset: egui::vec2(0.0, 4.0),
            blur: 16.0,
            spread: 0.0,
            color: egui::Color32::from_black_alpha(100),
        };

        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(45, 47, 50);
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(232, 234, 237);

        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(55, 58, 64);
        visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(232, 234, 237);
        visuals.widgets.inactive.rounding = 6.0.into();

        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(66, 70, 77);
        visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
        visuals.widgets.hovered.rounding = 6.0.into();

        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(66, 133, 244);
        visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;
        visuals.widgets.active.rounding = 6.0.into();

        visuals.selection.bg_fill = egui::Color32::from_rgb(66, 133, 244);
        visuals.selection.stroke.color = egui::Color32::from_rgb(138, 180, 248);

        visuals
    }

    pub(crate) fn get_light_theme(&self) -> egui::Visuals {
        let mut visuals = egui::Visuals::light();

        visuals.window_rounding = 8.0.into();
        visuals.window_shadow = egui::epaint::Shadow {
            offset: egui::vec2(0.0, 4.0),
            blur: 16.0,
            spread: 0.0,
            color: egui::Color32::from_black_alpha(50),
        };

        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(245, 245, 245);
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(50, 50, 50);

        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(230, 230, 230);
        visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(50, 50, 50);
        visuals.widgets.inactive.rounding = 6.0.into();

        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(200, 200, 200);
        visuals.widgets.hovered.fg_stroke.color = egui::Color32::BLACK;
        visuals.widgets.hovered.rounding = 6.0.into();

        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(66, 133, 244);
        visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;
        visuals.widgets.active.rounding = 6.0.into();

        visuals.selection.bg_fill = egui::Color32::from_rgb(66, 133, 244);
        visuals.selection.stroke.color = egui::Color32::from_rgb(100, 150, 255);

        visuals
    }

    pub(crate) fn get_dracula_theme(&self) -> egui::Visuals {
        let mut visuals = egui::Visuals::dark();

        visuals.window_rounding = 8.0.into();
        visuals.panel_fill = egui::Color32::from_rgb(40, 42, 54);
        visuals.window_fill = egui::Color32::from_rgb(40, 42, 54);

        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(68, 71, 90);
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(248, 248, 242);

        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(68, 71, 90);
        visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(248, 248, 242);
        visuals.widgets.inactive.rounding = 6.0.into();

        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(98, 114, 164);
        visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
        visuals.widgets.hovered.rounding = 6.0.into();

        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(189, 147, 249);
        visuals.widgets.active.fg_stroke.color = egui::Color32::BLACK;
        visuals.widgets.active.rounding = 6.0.into();

        visuals.selection.bg_fill = egui::Color32::from_rgb(189, 147, 249);
        visuals.selection.stroke.color = egui::Color32::from_rgb(255, 121, 198);

        visuals
    }

    pub(crate) fn get_nord_theme(&self) -> egui::Visuals {
        let mut visuals = egui::Visuals::dark();

        visuals.window_rounding = 8.0.into();
        visuals.panel_fill = egui::Color32::from_rgb(46, 52, 64);
        visuals.window_fill = egui::Color32::from_rgb(46, 52, 64);

        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(59, 66, 82);
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(236, 239, 244);

        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(67, 76, 94);
        visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(236, 239, 244);
        visuals.widgets.inactive.rounding = 6.0.into();

        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(76, 86, 106);
        visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
        visuals.widgets.hovered.rounding = 6.0.into();

        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(136, 192, 208);
        visuals.widgets.active.fg_stroke.color = egui::Color32::BLACK;
        visuals.widgets.active.rounding = 6.0.into();

        visuals.selection.bg_fill = egui::Color32::from_rgb(136, 192, 208);
        visuals.selection.stroke.color = egui::Color32::from_rgb(143, 188, 187);

        visuals
    }

    pub(crate) fn get_monokai_theme(&self) -> egui::Visuals {
        let mut visuals = egui::Visuals::dark();

        visuals.window_rounding = 8.0.into();
        visuals.panel_fill = egui::Color32::from_rgb(39, 40, 34);
        visuals.window_fill = egui::Color32::from_rgb(39, 40, 34);

        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(73, 72, 62);
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(248, 248, 242);

        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(73, 72, 62);
        visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(248, 248, 242);
        visuals.widgets.inactive.rounding = 6.0.into();

        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(102, 217, 239);
        visuals.widgets.hovered.fg_stroke.color = egui::Color32::BLACK;
        visuals.widgets.hovered.rounding = 6.0.into();

        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(166, 226, 46);
        visuals.widgets.active.fg_stroke.color = egui::Color32::BLACK;
        visuals.widgets.active.rounding = 6.0.into();

        visuals.selection.bg_fill = egui::Color32::from_rgb(166, 226, 46);
        visuals.selection.stroke.color = egui::Color32::from_rgb(249, 38, 114);

        visuals
    }

    pub(crate) fn get_solarized_dark_theme(&self) -> egui::Visuals {
        let mut visuals = egui::Visuals::dark();

        visuals.window_rounding = 8.0.into();
        visuals.panel_fill = egui::Color32::from_rgb(0, 43, 54);
        visuals.window_fill = egui::Color32::from_rgb(0, 43, 54);

        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(7, 54, 66);
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(131, 148, 150);

        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(7, 54, 66);
        visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(131, 148, 150);
        visuals.widgets.inactive.rounding = 6.0.into();

        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(88, 110, 117);
        visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
        visuals.widgets.hovered.rounding = 6.0.into();

        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(38, 139, 210);
        visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;
        visuals.widgets.active.rounding = 6.0.into();

        visuals.selection.bg_fill = egui::Color32::from_rgb(38, 139, 210);
        visuals.selection.stroke.color = egui::Color32::from_rgb(42, 161, 152);

        visuals
    }

}

/// Theme-owned colors for every manually painted UI element outside of
/// egui's built-in widget styling.
#[derive(Clone, Copy)]
pub(crate) struct Palette {
    pub bg: egui::Color32,
    pub hover_bg: egui::Color32,
    pub text: egui::Color32,
    pub text_dim: egui::Color32,
    pub accent: egui::Color32,
    pub success: egui::Color32,
    pub warning: egui::Color32,
    pub danger: egui::Color32,
    pub scrim: egui::Color32,
}

impl FileManagerApp {
    pub(crate) fn palette(&self) -> Palette {
        self.current_theme.palette()
    }
}

impl Theme {
    pub(crate) fn palette(&self) -> Palette {
        match self {
            Theme::Dark => Palette {
                bg: egui::Color32::from_rgb(45, 47, 50),
                hover_bg: egui::Color32::from_rgb(66, 70, 77),
                text: egui::Color32::from_rgb(232, 234, 237),
                text_dim: egui::Color32::from_rgb(154, 160, 166),
                accent: egui::Color32::from_rgb(66, 133, 244),
                success: egui::Color32::from_rgb(129, 201, 149),
                warning: egui::Color32::from_rgb(251, 188, 5),
                danger: egui::Color32::from_rgb(242, 139, 130),
                scrim: egui::Color32::from_black_alpha(100),
            },
            Theme::Light => Palette {
                bg: egui::Color32::from_rgb(245, 245, 245),
                hover_bg: egui::Color32::from_rgb(200, 200, 200),
                text: egui::Color32::from_rgb(50, 50, 50),
                text_dim: egui::Color32::from_rgb(110, 114, 120),
                accent: egui::Color32::from_rgb(66, 133, 244),
                success: egui::Color32::from_rgb(40, 167, 69),
                warning: egui::Color32::from_rgb(191, 144, 0),
                danger: egui::Color32::from_rgb(220, 53, 69),
                scrim: egui::Color32::from_black_alpha(50),
            },
            Theme::Dracula => Palette {
                bg: egui::Color32::from_rgb(40, 42, 54),
                hover_bg: egui::Color32::from_rgb(68, 71, 90),
                text: egui::Color32::from_rgb(248, 248, 242),
                text_dim: egui::Color32::from_rgb(98, 114, 164),
                accent: egui::Color32::from_rgb(189, 147, 249),
                success: egui::Color32::from_rgb(80, 250, 123),
                warning: egui::Color32::from_rgb(241, 250, 140),
                danger: egui::Color32::from_rgb(255, 85, 85),
                scrim: egui::Color32::from_black_alpha(100),
            },
            Theme::Nord => Palette {
                bg: egui::Color32::from_rgb(46, 52, 64),
                hover_bg: egui::Color32::from_rgb(67, 76, 94),
                text: egui::Color32::from_rgb(236, 239, 244),
                text_dim: egui::Color32::from_rgb(143, 158, 178),
                accent: egui::Color32::from_rgb(136, 192, 208),
                success: egui::Color32::from_rgb(163, 190, 140),
                warning: egui::Color32::from_rgb(235, 203, 139),
                danger: egui::Color32::from_rgb(191, 97, 106),
                scrim: egui::Color32::from_black_alpha(100),
            },
            Theme::Monokai => Palette {
                bg: egui::Color32::from_rgb(39, 40, 34),
                hover_bg: egui::Color32::from_rgb(73, 72, 62),
                text: egui::Color32::from_rgb(248, 248, 242),
                text_dim: egui::Color32::from_rgb(117, 113, 94),
                accent: egui::Color32::from_rgb(166, 226, 46),
                success: egui::Color32::from_rgb(166, 226, 46),
                warning: egui::Color32::from_rgb(230, 219, 116),
                danger: egui::Color32::from_rgb(249, 38, 114),
                scrim: egui::Color32::from_black_alpha(100),
            },
            Theme::SolarizedDark => Palette {
                bg: egui::Color32::from_rgb(7, 54, 66),
                hover_bg: egui::Color32::from_rgb(88, 110, 117),
                text: egui::Color32::from_rgb(238, 232, 213),
                text_dim: egui::Color32::from_rgb(131, 148, 150),
                accent: egui::Color32::from_rgb(38, 139, 210),
                success: egui::Color32::from_rgb(133, 153, 0),
                warning: egui::Color32::from_rgb(181, 137, 0),
                danger: egui::Color32::from_rgb(203, 75, 22),
                scrim: egui::Color32::from_black_alpha(100),
            },
        }
    }
}
