use super::FileManagerApp;
use egui;

impl FileManagerApp {
    pub(crate) fn render_delete_confirm(&mut self, ctx: &egui::Context) {
        // Delete confirmation dialog
        if self.show_delete_confirm {
            let name = self.item_to_delete.clone().unwrap_or_default();
            let permanent = self.delete_permanently;
            egui::Window::new("⚠  Delete Confirmation")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(egui::Color32::from_rgb(45, 47, 50))
                        .rounding(12.0)
                        .shadow(egui::epaint::Shadow {
                            offset: egui::vec2(0.0, 8.0),
                            blur: 24.0,
                            spread: 0.0,
                            color: egui::Color32::from_black_alpha(120),
                        })
                        .inner_margin(20.0),
                )
                .show(ctx, |ui| {
                    ui.style_mut()
                        .visuals
                        .widgets
                        .noninteractive
                        .fg_stroke
                        .color = egui::Color32::from_rgb(232, 234, 237);
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new("⚠")
                                .size(48.0)
                                .color(egui::Color32::from_rgb(251, 188, 5)),
                        );
                        ui.add_space(15.0);
                        ui.label(
                            egui::RichText::new(if permanent {
                                "Delete permanently?"
                            } else {
                                "Move this item to Trash?"
                            })
                            .size(16.0)
                            .strong(),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(if permanent {
                                "This cannot be undone!"
                            } else {
                                "You can restore it later from the Trash"
                            })
                            .size(12.0)
                            .color(if permanent {
                                egui::Color32::from_rgb(251, 100, 90)
                            } else {
                                egui::Color32::from_rgb(160, 164, 170)
                            }),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(&name)
                                .size(14.0)
                                .color(egui::Color32::from_rgb(138, 180, 248)),
                        );
                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            if                            ui.add_sized(
                                    [120.0, 36.0],
                                    egui::Button::new(egui::RichText::new(if permanent {
                                        "🗑  Delete Permanently"
                                    } else {
                                        "✓  Move to Trash"
                                    })
                                    .size(14.0))
                                    .fill(egui::Color32::from_rgb(220, 53, 69))
                                    .rounding(6.0),
                                )
                                .clicked()
                            {
                                let _ = self.delete_selected_items(permanent);
                                self.show_delete_confirm = false;
                                self.delete_permanently = false;
                                self.item_to_delete = None;
                            }
                            ui.add_space(12.0);
                            if ui
                                .add_sized(
                                    [120.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✗  Cancel").size(14.0))
                                        .fill(egui::Color32::from_rgb(66, 70, 77))
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                self.show_delete_confirm = false;
                                self.delete_permanently = false;
                                self.item_to_delete = None;
                            }
                        });
                        ui.add_space(10.0);
                    });
                });
        }

    }

    /// Conflict dialog shown when pasting into a destination where an item
    /// with the same name already exists.
    pub(crate) fn render_paste_conflict_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_paste_conflict {
            return;
        }
        let source_name = self.paste_conflict_source_name.clone();
        let new_name = self.paste_conflict_new_name.clone();

        egui::Window::new("⚠  File already exists")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(egui::Color32::from_rgb(45, 47, 50))
                    .rounding(12.0)
                    .shadow(egui::epaint::Shadow {
                        offset: egui::vec2(0.0, 8.0),
                        blur: 24.0,
                        spread: 0.0,
                        color: egui::Color32::from_black_alpha(120),
                    })
                    .inner_margin(20.0),
            )
            .show(ctx, |ui| {
                ui.style_mut()
                    .visuals
                    .widgets
                    .noninteractive
                    .fg_stroke
                    .color = egui::Color32::from_rgb(232, 234, 237);
                ui.set_min_width(360.0);
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(format!("Item '{}' already exists here.", source_name))
                            .size(15.0)
                            .strong(),
                    );
                    ui.add_space(12.0);

                    // Rename option
                    ui.label(egui::RichText::new("Paste as:").size(13.0));
                    ui.add_space(4.0);
                    let rename_field = ui.add(
                        egui::TextEdit::singleline(&mut self.paste_conflict_new_name)
                            .hint_text("new name")
                            .desired_width(f32::INFINITY),
                    );
                    // Return inside the rename field = Rename & Paste
                    let rename_pressed = rename_field.lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                [130.0, 34.0],
                                egui::Button::new(egui::RichText::new("⤴  Overwrite").size(13.0))
                                    .fill(egui::Color32::from_rgb(255, 152, 0))
                                    .rounding(6.0),
                            )
                            .clicked()
                        {
                            self.paste_conflict_overwrite();
                        }
                        if ui
                            .add_sized(
                                [130.0, 34.0],
                                egui::Button::new(egui::RichText::new("✏  Rename").size(13.0))
                                    .fill(egui::Color32::from_rgb(66, 133, 244))
                                    .rounding(6.0),
                            )
                            .clicked()
                            || rename_pressed
                        {
                            self.paste_conflict_rename();
                        }
                    });
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                [130.0, 34.0],
                                egui::Button::new(egui::RichText::new("⏭  Skip").size(13.0))
                                    .fill(egui::Color32::from_rgb(66, 70, 77))
                                    .rounding(6.0),
                            )
                            .clicked()
                        {
                            self.paste_conflict_skip();
                        }
                        if ui
                            .add_sized(
                                [130.0, 34.0],
                                egui::Button::new(egui::RichText::new("✗  Cancel").size(13.0))
                                    .fill(egui::Color32::from_rgb(66, 70, 77))
                                    .rounding(6.0),
                            )
                            .clicked()
                        {
                            self.paste_conflict_cancel();
                        }
                    });

                    let _ = new_name; // new_name lives in paste_conflict_new_name
                });
            });
    }

    pub(crate) fn render_progress_dialog(&mut self, ctx: &egui::Context) {
        // Progress dialog for background file operations
        if self.show_progress_dialog {
            let snap = self.op_runner.as_ref().map(|r| r.snapshot());
            if let Some(snap) = snap {
                let running = !snap.finished;
                let byte_pct = if snap.bytes_total > 0 {
                    (snap.bytes_done as f32 / snap.bytes_total as f32).min(1.0)
                } else if snap.finished {
                    1.0
                } else {
                    0.0
                };

                let (icon, color) = if let Some(_err) = &snap.error {
                    ("❌".to_string(), egui::Color32::from_rgb(242, 139, 130))
                } else if snap.cancelled {
                    ("⚠️".to_string(), egui::Color32::from_rgb(251, 188, 5))
                } else if snap.finished {
                    ("✅".to_string(), egui::Color32::from_rgb(129, 201, 149))
                } else {
                    (snap.kind.icon().to_string(), egui::Color32::from_rgb(138, 180, 248))
                };

                let err_text = snap.error.clone();

                egui::Window::new(format!("{}  {}", icon, snap.kind.label()))
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .frame(
                        egui::Frame::window(&ctx.style())
                            .fill(egui::Color32::from_rgb(45, 47, 50))
                            .rounding(12.0)
                            .shadow(egui::epaint::Shadow {
                                offset: egui::vec2(0.0, 8.0),
                                blur: 24.0,
                                spread: 0.0,
                                color: egui::Color32::from_black_alpha(120),
                            })
                            .inner_margin(20.0),
                    )
                    .show(ctx, |ui| {
                        ui.style_mut()
                            .visuals
                            .widgets
                            .noninteractive
                            .fg_stroke
                            .color = egui::Color32::from_rgb(232, 234, 237);
                        ui.set_min_width(420.0);
                        ui.vertical(|ui| {
                            ui.add_space(6.0);
                            ui.label(
                                egui::RichText::new(&snap.title)
                                    .size(15.0)
                                    .strong()
                                    .color(color),
                            );
                            ui.add_space(4.0);
                            if running {
                                ui.label(
                                    egui::RichText::new(format!("📄 {}", snap.current_file))
                                        .size(12.0)
                                        .color(egui::Color32::from_rgb(160, 164, 170)),
                                );
                            }
                            ui.add_space(10.0);
                            ui.add(
                                egui::ProgressBar::new(byte_pct).show_percentage(),
                            );
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{} / {} files",
                                        snap.files_done, snap.files_total
                                    ))
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(160, 164, 170)),
                                );
                                if snap.bytes_total > 0 {
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "{:.1} MB / {:.1} MB",
                                                snap.bytes_done as f64 / 1_048_576.0,
                                                snap.bytes_total as f64 / 1_048_576.0
                                            ))
                                            .size(12.0)
                                            .color(egui::Color32::from_rgb(160, 164, 170)),
                                        );
                                    });
                                }
                            });
                            ui.add_space(14.0);
                            ui.horizontal(|ui| {
                                if running {
                                    if ui
                                        .add_sized(
                                            [120.0, 34.0],
                                            egui::Button::new(
                                                egui::RichText::new("✗  Cancel").size(13.0),
                                            )
                                            .fill(egui::Color32::from_rgb(220, 53, 69))
                                            .rounding(6.0),
                                        )
                                        .clicked()
                                    {
                                        if let Some(runner) = &self.op_runner {
                                            runner.cancel();
                                        }
                                    }
                                } else {
                                    let msg = err_text.clone().unwrap_or_else(|| {
                                        if snap.cancelled {
                                            "Operation was cancelled".to_string()
                                        } else {
                                            "Completed successfully".to_string()
                                        }
                                    });
                                    ui.label(
                                        egui::RichText::new(msg)
                                            .size(13.0)
                                            .color(color),
                                    );
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui
                                            .add_sized(
                                                [100.0, 34.0],
                                                egui::Button::new(
                                                    egui::RichText::new("Close").size(13.0),
                                                )
                                                .fill(egui::Color32::from_rgb(66, 70, 77))
                                                .rounding(6.0),
                                            )
                                            .clicked()
                                        {
                                            self.show_progress_dialog = false;
                                            self.op_result_message = None;
                                        }
                                    });
                                }
                            });
                            ui.add_space(4.0);
                        });
                    });
            } else {
                // Runner gone but dialog still shown - close it
                self.show_progress_dialog = false;
            }
        }
        
    }

}
