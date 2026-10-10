use super::FileManagerApp;
use egui;

impl FileManagerApp {
    pub(crate) fn render_sidebar_widgets(&mut self, ctx: &egui::Context) {
        let visuals = ctx.style().visuals.clone();
        // Sidebar (Left Panel)
        if self.show_sidebar {
            egui::SidePanel::left("sidebar")
                .resizable(false)
                .exact_width(220.0)
                .frame(
                    egui::Frame::default()
                        .fill(visuals.panel_fill)
                        .inner_margin(8.0),
                )
                .show(ctx, |ui| {
                    self.render_sidebar(ui, ctx);
                });
        }

        // Sidebar collapse button (floating on left edge)
        if self.show_sidebar {
            egui::Area::new(egui::Id::new("sidebar_collapse_btn"))
                .fixed_pos(egui::pos2(220.0, 100.0))
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    if ui
                        .add_sized(
                            [24.0, 48.0],
                            egui::Button::new(egui::RichText::new("◀").size(14.0).color(visuals.widgets.active.fg_stroke.color))
                                .fill(visuals.widgets.active.bg_fill)
                                .rounding(egui::Rounding {
                                    nw: 0.0,
                                    ne: 8.0,
                                    sw: 0.0,
                                    se: 8.0,
                                }),
                        )
                        .on_hover_text("Hide sidebar (Ctrl+B)")
                        .clicked()
                    {
                        self.show_sidebar = false;
                    }
                });
        } else {
            // Sidebar expand button (when sidebar is hidden)
            egui::Area::new(egui::Id::new("sidebar_expand_btn"))
                .fixed_pos(egui::pos2(0.0, 100.0))
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    if ui
                        .add_sized(
                            [24.0, 48.0],
                            egui::Button::new(egui::RichText::new("▶").size(14.0).color(visuals.widgets.active.fg_stroke.color))
                                .fill(visuals.widgets.active.bg_fill)
                                .rounding(egui::Rounding {
                                    nw: 0.0,
                                    ne: 8.0,
                                    sw: 0.0,
                                    se: 8.0,
                                }),
                        )
                        .on_hover_text("Show sidebar (Ctrl+B)")
                        .clicked()
                    {
                        self.show_sidebar = true;
                    }
                });
        }

    }
}
