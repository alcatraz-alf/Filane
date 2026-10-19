use super::FileManagerApp;
use egui;

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
                            ui.set_min_width(180.0);
                            ui.style_mut().spacing.button_padding = egui::vec2(10.0, 8.0);
                            ui.style_mut().visuals.widgets.inactive.fg_stroke.color =
                                egui::Color32::from_rgb(232, 234, 237);
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                                egui::Color32::from_rgb(66, 70, 77);
                            ui.style_mut().visuals.widgets.hovered.fg_stroke.color =
                                egui::Color32::WHITE;

                            if let Some(item) = &item {
                                // Restore option when browsing inside the trash
                                if item.name != ".."
                                    && crate::trash::is_trash_path(&self.get_active_pane().current_path.clone())
                                {                                    if ui
                                        .add_sized(
                                            [ui.available_width(), 28.0],
                                            egui::Button::new(
                                                egui::RichText::new("↩  Restore to original location")
                                                    .size(13.0),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        self.restore_trashed_item(&item.path, &item.name);
                                        close_menu = true;
                                    }
                                    ui.separator();
                                }

                                    if ui
                                        .add_sized(
                                            [ui.available_width(), 28.0],
                                            egui::Button::new(
                                                egui::RichText::new("🖥  Open Terminal Here")
                                                    .size(13.0),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        let _ = self.open_terminal_here();
                                        close_menu = true;
                                    }
                                    ui.separator();

                                // Open file option (for non-directories)
                                if !item.is_dir && item.name != ".." {
                                    if ui
                                        .add_sized(
                                            [ui.available_width(), 28.0],
                                            egui::Button::new(
                                                egui::RichText::new("📄  Open").size(13.0),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        let _ = self.open_file_with_default_app();
                                        close_menu = true;
                                    }
                                    ui.separator();
                                }

                                // Properties
                                if item.name != ".." {
                                    if ui
                                        .add_sized(
                                            [ui.available_width(), 28.0],
                                            egui::Button::new(
                                                egui::RichText::new("ℹ  Properties").size(13.0),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        self.properties_item = Some(item.clone());
                                        self.show_properties_dialog = true;
                                        close_menu = true;
                                    }
                                    ui.separator();
                                }

                                // Rename
                                if item.name != ".." {
                                    if ui
                                        .add_sized(
                                            [ui.available_width(), 28.0],
                                            egui::Button::new(
                                                egui::RichText::new("✏  Rename").size(13.0),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        self.rename_new_name = item.name.clone();
                                        self.show_rename_dialog = true;
                                        close_menu = true;
                                    }

                                    ui.separator();

                                    // Copy
                                    if ui
                                        .add_sized(
                                            [ui.available_width(), 28.0],
                                            egui::Button::new(
                                                egui::RichText::new("📋  Copy").size(13.0),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        self.copy_to_clipboard();
                                        close_menu = true;
                                    }

                                    // Cut
                                    if ui
                                        .add_sized(
                                            [ui.available_width(), 28.0],
                                            egui::Button::new(
                                                egui::RichText::new("✂  Cut").size(13.0),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        self.cut_to_clipboard();
                                        close_menu = true;
                                    }

                                    ui.separator();

                                    // Compress (for files and folders, not "..")
                                    if ui
                                        .add_sized(
                                            [ui.available_width(), 28.0],
                                            egui::Button::new(
                                                egui::RichText::new("🗃️  Compress to ZIP")
                                                    .size(13.0),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        let _ = self.compress_item(super::ArchiveKind::Zip);
                                        close_menu = true;
                                    }

                                    // Compress to TAR.GZ
                                    if ui
                                        .add_sized(
                                            [ui.available_width(), 28.0],
                                            egui::Button::new(
                                                egui::RichText::new("🗜️  Compress to TAR.GZ")
                                                    .size(13.0),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        let _ = self.compress_item(super::ArchiveKind::TarGz);
                                        close_menu = true;
                                    }

                                    // Extract (for zip and tar/tar.gz archives)
                                    if !item.is_dir
                                        && (item.name.ends_with(".zip")
                                            || crate::filesystem::is_tar_gz(&item.path)
                                            || crate::filesystem::is_tar(&item.path))
                                    {
                                        if ui
                                            .add_sized(
                                                [ui.available_width(), 28.0],
                                                egui::Button::new(
                                                    egui::RichText::new("📦  Extract Archive")
                                                        .size(13.0),
                                                )
                                                .frame(false),
                                            )
                                            .clicked()
                                        {
                                            let _ = self.extract_archive();
                                            close_menu = true;
                                        }
                                    }

                                    ui.separator();

                                    // Delete
                                    if ui
                                        .add_sized(
                                            [ui.available_width(), 28.0],
                                            egui::Button::new(
                                                egui::RichText::new("🗑  Delete")
                                                    .size(13.0)
                                                    .color(egui::Color32::from_rgb(242, 139, 130)),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        self.item_to_delete = Some(item.name.clone());
                                        self.delete_permanently = false;
                                        self.show_delete_confirm = true;
                                        close_menu = true;
                                    }

                                    // Delete Permanently (bypasses trash)
                                    if ui
                                        .add_sized(
                                            [ui.available_width(), 28.0],
                                            egui::Button::new(
                                                egui::RichText::new("🗑  Delete Permanently (Shift+Del)")
                                                    .size(13.0)
                                                    .color(egui::Color32::from_rgb(220, 80, 70)),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        self.item_to_delete = Some(item.name.clone());
                                        self.delete_permanently = true;
                                        self.show_delete_confirm = true;
                                        close_menu = true;
                                    }
                                }

                                ui.separator();

                                // Cancel
                                if ui
                                    .add_sized(
                                        [ui.available_width(), 28.0],
                                        egui::Button::new(
                                            egui::RichText::new("✗  Cancel").size(13.0),
                                        )
                                        .frame(false),
                                    )
                                    .clicked()
                                {
                                    close_menu = true;
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
