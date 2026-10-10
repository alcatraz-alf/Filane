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
