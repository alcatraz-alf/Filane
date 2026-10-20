use super::FileManagerApp;
use egui;

/// Full-width, evenly styled context menu row with an optional right-aligned
/// keyboard shortcut hint (e.g. "Ctrl+C").
fn menu_btn(
    ui: &mut egui::Ui,
    icon: &str,
    label: &str,
    hint: Option<&str>,
    danger: bool,
) -> egui::Response {
    let width = ui.available_width();
    let text_color = if danger {
        egui::Color32::from_rgb(240, 146, 138)
    } else {
        egui::Color32::from_rgb(232, 234, 237)
    };
    let hint_color = egui::Color32::from_rgb(140, 144, 150);

    egui::Frame::none()
        .inner_margin(egui::Margin::symmetric(8.0, 6.0))
        .show(ui, |ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(width - 16.0, 18.0), egui::Sense::hover());
            let pos = rect.left_top();
            ui.painter()
                .text(pos, egui::Align2::LEFT_CENTER, icon, egui::FontId::proportional(13.0), text_color);
            ui.painter().text(
                pos + egui::vec2(24.0, 0.0),
                egui::Align2::LEFT_CENTER,
                label,
                egui::FontId::proportional(13.0),
                text_color,
            );
            if let Some(h) = hint {
                ui.painter().text(
                    pos + egui::vec2(width - 32.0, 0.0),
                    egui::Align2::RIGHT_CENTER,
                    h,
                    egui::FontId::monospace(11.0),
                    hint_color,
                );
            }
            let response = ui.interact(rect, egui::Id::new(icon.to_string() + label), egui::Sense::click());
            if response.hovered() {
                ui.painter()
                    .rect_filled(rect, 6.0, egui::Color32::from_rgb(66, 70, 77));
            }
            response
        })
        .inner
}

impl FileManagerApp {
    pub(crate) fn render_context_menu(&mut self, ctx: &egui::Context) {
        // Context menu (right-click menu)
        if self.show_context_menu {
            let item = self.get_active_pane().get_selected_item().cloned();
            let mut close_menu = false;

            let menu_response = egui::Area::new(egui::Id::new("context_menu"))
                .fixed_pos(self.context_menu_pos)
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    egui::Frame::popup(&ctx.style())
                        .fill(egui::Color32::from_rgb(45, 47, 50))
                        .rounding(8.0)
                        .shadow(egui::epaint::Shadow {
                            offset: egui::vec2(0.0, 4.0),
                            blur: 16.0,
                            spread: 0.0,
                            color: egui::Color32::from_black_alpha(100),
                        })
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.set_min_width(240.0);
                            ui.spacing_mut().item_spacing.y = 1.0;
                            ui.style_mut().visuals.widgets.inactive.fg_stroke.color =
                                egui::Color32::from_rgb(232, 234, 237);

                            if let Some(item) = &item {
                                let not_dotdot = item.name != "..";

                                // --- Open group -----------------------------------
                                if !item.is_dir && not_dotdot {
                                    if menu_btn(ui, "📄", "Open", Some("F3"), false).clicked() {
                                        let _ = self.open_file_with_default_app();
                                        close_menu = true;
                                    }
                                }
                                if menu_btn(ui, "🖥", "Open Terminal Here", Some("Ctrl+T"), false)
                                    .clicked()
                                {
                                    let _ = self.open_terminal_here();
                                    close_menu = true;
                                }

                                ui.separator();

                                // --- Clipboard / editing group -------------------
                                if not_dotdot {
                                    if menu_btn(ui, "📋", "Copy", Some("Ctrl+C"), false).clicked() {
                                        self.copy_to_clipboard();
                                        close_menu = true;
                                    }
                                    if menu_btn(ui, "✂", "Cut", Some("Ctrl+X"), false).clicked() {
                                        self.cut_to_clipboard();
                                        close_menu = true;
                                    }
                                    if self.clipboard.is_some()
                                        && menu_btn(ui, "📌", "Paste", Some("Ctrl+V"), false).clicked()
                                    {
                                        let _ = self.paste_from_clipboard();
                                        close_menu = true;
                                    }
                                    if menu_btn(ui, "✏", "Rename", Some("Shift+F2"), false)
                                        .clicked()
                                    {
                                        self.rename_new_name = item.name.clone();
                                        self.show_rename_dialog = true;
                                        close_menu = true;
                                    }

                                    // --- Archive group -------------------------------
                                    ui.separator();
                                    if menu_btn(ui, "🗃", "Compress to ZIP", None, false).clicked() {
                                        let _ = self.compress_item(super::ArchiveKind::Zip);
                                        close_menu = true;
                                    }
                                    if menu_btn(ui, "🗜", "Compress to TAR.GZ", None, false)
                                        .clicked()
                                    {
                                        let _ = self.compress_item(super::ArchiveKind::TarGz);
                                        close_menu = true;
                                    }
                                    if !item.is_dir
                                        && (item.name.ends_with(".zip")
                                            || crate::filesystem::is_tar_gz(&item.path)
                                            || crate::filesystem::is_tar(&item.path))
                                        && menu_btn(ui, "📦", "Extract Archive", None, false)
                                            .clicked()
                                    {
                                        let _ = self.extract_archive();
                                        close_menu = true;
                                    }

                                    // --- Link ----------------------------------------
                                    #[cfg(unix)]
                                    if menu_btn(
                                        ui,
                                        "🔗",
                                        "Symlink in other pane",
                                        None,
                                        false,
                                    )
                                    .clicked()
                                    {
                                        let _ = self.symlink_in_other_pane();
                                        close_menu = true;
                                    }

                                    // --- Destructive group ---------------------------
                                    ui.separator();
                                    if menu_btn(ui, "🗑", "Delete", Some("Del"), true).clicked() {
                                        self.item_to_delete = Some(item.name.clone());
                                        self.delete_permanently = false;
                                        self.show_delete_confirm = true;
                                        close_menu = true;
                                    }
                                    if menu_btn(
                                        ui,
                                        "🗑",
                                        "Delete Permanently",
                                        Some("Shift+Del"),
                                        true,
                                    )
                                    .clicked()
                                    {
                                        self.item_to_delete = Some(item.name.clone());
                                        self.delete_permanently = true;
                                        self.show_delete_confirm = true;
                                        close_menu = true;
                                    }

                                    // --- Info ----------------------------------------
                                    ui.separator();
                                    if menu_btn(ui, "ℹ", "Properties", None, false).clicked() {
                                        self.properties_item = Some(item.clone());
                                        self.chmod_input =
                                            crate::filesystem::get_mode_octal(&item.path)
                                                .map(|m| format!("{:o}", m))
                                                .unwrap_or_default();
                                        self.show_properties_dialog = true;
                                        close_menu = true;
                                    }
                                }

                                // --- Trash restore (when browsing trash) ---------
                                if not_dotdot
                                    && crate::trash::is_trash_path(
                                        &self.get_active_pane().current_path,
                                    )
                                {
                                    ui.separator();
                                    if menu_btn(
                                        ui,
                                        "↩",
                                        "Restore to original location",
                                        None,
                                        false,
                                    )
                                    .clicked()
                                    {
                                        self.restore_trashed_item(&item.path, &item.name);
                                        close_menu = true;
                                    }
                                }
                            }
                        });
                });

            // Close menu if action was clicked
            if close_menu {
                self.show_context_menu = false;
                self.context_menu_just_opened = false;
            }

            // Close on click outside - only check primary click (left click)
            // But not if the menu was just opened this frame
            if !self.context_menu_just_opened && ctx.input(|i| i.pointer.primary_clicked()) {
                let menu_rect = menu_response.response.rect;
                if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
                    if !menu_rect.contains(pos) {
                        self.show_context_menu = false;
                    }
                }
            }

            // Reset the "just opened" flag after one frame
            if self.context_menu_just_opened {
                self.context_menu_just_opened = false;
            }

            // Close on ESC key
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.show_context_menu = false;
            }
        }
    }
}
