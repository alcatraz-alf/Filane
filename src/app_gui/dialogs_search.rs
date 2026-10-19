use super::FileManagerApp;
use egui;

impl FileManagerApp {
    pub(crate) fn render_search_dialog(&mut self, ctx: &egui::Context) {
        let visuals = ctx.style().visuals.clone();
        // Advanced Search Dialog
        if self.show_search_dialog {
            egui::Window::new("🔍 Advanced Search")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(egui::Color32::from_rgb(45, 47, 50))
                        .rounding(12.0)
                        .inner_margin(20.0),
                )
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(5.0);

                        egui::Grid::new("search_grid")
                            .num_columns(2)
                            .spacing([15.0, 10.0])
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("Search in:").strong().size(13.0));
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(
                                            self.search_criteria.search_path.display().to_string(),
                                        )
                                        .size(12.0)
                                        .color(visuals.widgets.active.bg_fill),
                                    );
                                    if ui.small_button("📁").clicked() {
                                        self.search_criteria.search_path =
                                            self.get_active_pane().current_path.clone();
                                    }
                                });
                                ui.end_row();

                                ui.label(egui::RichText::new("File name:").size(13.0));
                                ui.add_sized(
                                    [300.0, 24.0],
                                    egui::TextEdit::singleline(
                                        &mut self.search_criteria.filename_pattern,
                                    )
                                    .hint_text("e.g., *.rs, document, photo*"),
                                );
                                ui.end_row();

                                ui.label(egui::RichText::new("Contains text:").size(13.0));
                                ui.add_sized(
                                    [300.0, 24.0],
                                    egui::TextEdit::singleline(
                                        &mut self.search_criteria.content_pattern,
                                    )
                                    .hint_text("Search file contents"),
                                );
                                ui.end_row();

                                ui.label(egui::RichText::new("File type:").size(13.0));
                                ui.horizontal(|ui| {
                                    ui.radio_value(
                                        &mut self.search_criteria.file_type,
                                        crate::filesystem::SearchFileType::All,
                                        "All",
                                    );
                                    ui.radio_value(
                                        &mut self.search_criteria.file_type,
                                        crate::filesystem::SearchFileType::Files,
                                        "Files",
                                    );
                                    ui.radio_value(
                                        &mut self.search_criteria.file_type,
                                        crate::filesystem::SearchFileType::Directories,
                                        "Folders",
                                    );
                                });
                                ui.end_row();

                                ui.label(egui::RichText::new("Size (KB):").size(13.0));
                                ui.horizontal(|ui| {
                                    ui.label("Min:");
                                    ui.add_sized(
                                        [80.0, 24.0],
                                        egui::TextEdit::singleline(&mut self.search_min_size_text)
                                            .hint_text("0"),
                                    );
                                    ui.add_space(10.0);
                                    ui.label("Max:");
                                    ui.add_sized(
                                        [80.0, 24.0],
                                        egui::TextEdit::singleline(&mut self.search_max_size_text)
                                            .hint_text("∞"),
                                    );
                                });
                                ui.end_row();

                                ui.label(egui::RichText::new("Modified:").size(13.0));
                                ui.horizontal(|ui| {
                                    ui.label("Last");
                                    ui.add_sized(
                                        [60.0, 24.0],
                                        egui::TextEdit::singleline(&mut self.search_days_ago)
                                            .hint_text("7"),
                                    );
                                    ui.label("days");
                                });
                                ui.end_row();

                                ui.label(egui::RichText::new("Options:").size(13.0));
                                ui.vertical(|ui| {
                                    ui.checkbox(
                                        &mut self.search_criteria.case_sensitive,
                                        "Case sensitive",
                                    );
                                    ui.checkbox(
                                        &mut self.search_criteria.include_hidden,
                                        "Include hidden files",
                                    );
                                });
                                ui.end_row();
                            });

                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(10.0);

                        if self.search_in_progress {
                            ui.horizontal(|ui| {
                                ui.spinner();
                                ui.label(egui::RichText::new("Searching...").size(13.0));
                            });
                            ui.add_space(10.0);
                        }

                        if !self.search_results.is_empty() {
                            ui.label(
                                egui::RichText::new(format!(
                                    "Found {} results",
                                    self.search_results.len()
                                ))
                                .size(13.0)
                                .color(visuals.widgets.active.bg_fill),
                            );
                            ui.add_space(5.0);

                            let results_clone = self.search_results.clone();
                            let mut go_to: Option<&crate::filesystem::FileItem> = None;

                            egui::ScrollArea::vertical()
                                .max_height(200.0)
                                .show(ui, |ui| {
                                    for item in &results_clone {
                                        let icon = if item.is_dir { "📁" } else { "📄" };
                                        let text = format!("{} {}", icon, item.path.display());
                                        if ui
                                            .selectable_label(
                                                false,
                                                egui::RichText::new(text).size(11.0),
                                            )
                                            .clicked()
                                        {
                                            go_to = Some(item);
                                        }
                                    }
                                });

                            if let Some(item) = go_to {
                                let _ = self
                                    .get_active_pane_mut()
                                    .navigate_to_containing(&item.path);
                                self.show_search_dialog = false;
                            }

                            ui.add_space(10.0);
                        }

                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("🔍 Search").size(14.0))
                                        .fill(egui::Color32::from_rgb(66, 133, 244))
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                if !self.search_min_size_text.is_empty() {
                                    if let Ok(kb) = self.search_min_size_text.parse::<u64>() {
                                        self.search_criteria.min_size = Some(kb * 1024);
                                    }
                                }
                                if !self.search_max_size_text.is_empty() {
                                    if let Ok(kb) = self.search_max_size_text.parse::<u64>() {
                                        self.search_criteria.max_size = Some(kb * 1024);
                                    }
                                }
                                if !self.search_days_ago.is_empty() {
                                    if let Ok(days) = self.search_days_ago.parse::<u64>() {
                                        let duration =
                                            std::time::Duration::from_secs(days * 24 * 60 * 60);
                                        self.search_criteria.modified_after =
                                            Some(std::time::SystemTime::now() - duration);
                                    }
                                }

                                self.search_in_progress = true;
                                match crate::filesystem::search_files(&self.search_criteria) {
                                    Ok(results) => {
                                        self.search_results = results;
                                        self.status_message = format!(
                                            "Search complete: {} results",
                                            self.search_results.len()
                                        );
                                    }
                                    Err(e) => {
                                        self.status_message = format!("Search error: {}", e);
                                    }
                                }
                                self.search_in_progress = false;
                            }

                            ui.add_space(10.0);

                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✗ Close").size(14.0))
                                        .fill(egui::Color32::from_rgb(66, 70, 77))
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                self.show_search_dialog = false;
                                self.search_results.clear();
                            }
                        });
                    });
                });
        }

    }

    pub(crate) fn render_compare_dialog(&mut self, ctx: &egui::Context) {
        // File Comparison Dialog
        if self.show_compare_dialog {
            if let Some(comparison) = self.comparison_result.clone() {
                let mut close_dialog = false;

                egui::Window::new("⚖ File Comparison")
                    .collapsible(false)
                    .resizable(true)
                    .default_size([1000.0, 600.0])
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .frame(
                        egui::Frame::window(&ctx.style())
                            .fill(egui::Color32::from_rgb(45, 47, 50))
                            .rounding(12.0)
                            .inner_margin(15.0),
                    )
                    .show(ctx, |ui| {
                        ui.vertical(|ui| {
                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Left:").strong().size(12.0));
                                ui.label(
                                    egui::RichText::new(comparison.left_path.display().to_string())
                                        .size(11.0)
                                        .color(egui::Color32::from_rgb(138, 180, 248)),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Right:").strong().size(12.0));
                                ui.label(
                                    egui::RichText::new(
                                        comparison.right_path.display().to_string(),
                                    )
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(138, 180, 248)),
                                );
                            });

                            ui.add_space(10.0);

                            if comparison.are_identical {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("✅").size(24.0));
                                    ui.label(
                                        egui::RichText::new("Files are identical")
                                            .size(14.0)
                                            .color(egui::Color32::from_rgb(129, 201, 149)),
                                    );
                                });
                            } else {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new("Statistics:").strong().size(12.0),
                                    );
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Equal: {} | Added: {} | Removed: {} | Modified: {}",
                                            comparison.equal_lines,
                                            comparison.right_only_lines,
                                            comparison.left_only_lines,
                                            comparison.modified_lines
                                        ))
                                        .size(11.0),
                                    );
                                });

                                ui.add_space(5.0);
                                ui.separator();
                                ui.add_space(5.0);

                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("Legend:").strong().size(11.0));
                                    ui.label(egui::RichText::new("⬜ Equal").size(10.0));
                                    ui.label(
                                        egui::RichText::new("🟩 Added")
                                            .size(10.0)
                                            .color(egui::Color32::from_rgb(129, 201, 149)),
                                    );
                                    ui.label(
                                        egui::RichText::new("🟥 Removed")
                                            .size(10.0)
                                            .color(egui::Color32::from_rgb(242, 139, 130)),
                                    );
                                    ui.label(
                                        egui::RichText::new("🟨 Modified")
                                            .size(10.0)
                                            .color(egui::Color32::from_rgb(255, 193, 7)),
                                    );
                                });

                                ui.add_space(10.0);

                                egui::ScrollArea::vertical()
                                    .max_height(400.0)
                                    .show(ui, |ui| {
                                        ui.style_mut().spacing.item_spacing.y = 1.0;

                                        for diff_line in &comparison.diff_lines {
                                            let (bg_color, icon) = match diff_line.line_type {
                                                crate::filesystem::DiffLineType::Equal => {
                                                    (egui::Color32::from_rgb(50, 52, 55), "⬜")
                                                }
                                                crate::filesystem::DiffLineType::Added => {
                                                    (egui::Color32::from_rgb(40, 80, 40), "🟩")
                                                }
                                                crate::filesystem::DiffLineType::Removed => {
                                                    (egui::Color32::from_rgb(80, 40, 40), "🟥")
                                                }
                                                crate::filesystem::DiffLineType::Modified => {
                                                    (egui::Color32::from_rgb(80, 70, 30), "🟨")
                                                }
                                            };

                                            egui::Frame::default()
                                                .fill(bg_color)
                                                .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                                .show(ui, |ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            egui::RichText::new(icon).size(10.0),
                                                        );

                                                        let left_num = diff_line
                                                            .left_line_num
                                                            .map(|n| format!("{:4}", n))
                                                            .unwrap_or_else(|| "    ".to_string());
                                                        ui.label(
                                                            egui::RichText::new(left_num)
                                                                .size(10.0)
                                                                .family(egui::FontFamily::Monospace)
                                                                .color(egui::Color32::from_rgb(
                                                                    154, 160, 166,
                                                                )),
                                                        );

                                                        ui.label(
                                                            egui::RichText::new(
                                                                &diff_line.left_content,
                                                            )
                                                            .size(10.0)
                                                            .family(egui::FontFamily::Monospace),
                                                        );

                                                        ui.add_space(20.0);
                                                        ui.label(
                                                            egui::RichText::new("|")
                                                                .size(10.0)
                                                                .color(egui::Color32::from_rgb(
                                                                    100, 100, 100,
                                                                )),
                                                        );
                                                        ui.add_space(20.0);

                                                        let right_num = diff_line
                                                            .right_line_num
                                                            .map(|n| format!("{:4}", n))
                                                            .unwrap_or_else(|| "    ".to_string());
                                                        ui.label(
                                                            egui::RichText::new(right_num)
                                                                .size(10.0)
                                                                .family(egui::FontFamily::Monospace)
                                                                .color(egui::Color32::from_rgb(
                                                                    154, 160, 166,
                                                                )),
                                                        );

                                                        ui.label(
                                                            egui::RichText::new(
                                                                &diff_line.right_content,
                                                            )
                                                            .size(10.0)
                                                            .family(egui::FontFamily::Monospace),
                                                        );
                                                    });
                                                });
                                        }
                                    });
                            }

                            ui.add_space(15.0);
                            ui.separator();
                            ui.add_space(10.0);

                            ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() / 2.0 - 100.0);
                                if ui
                                    .add_sized(
                                        [200.0, 36.0],
                                        egui::Button::new(
                                            egui::RichText::new("Close (ESC)").size(14.0),
                                        )
                                        .fill(egui::Color32::from_rgb(66, 70, 77))
                                        .rounding(6.0),
                                    )
                                    .clicked()
                                {
                                    close_dialog = true;
                                }
                            });
                        });
                    });

                if close_dialog {
                    self.show_compare_dialog = false;
                    self.comparison_result = None;
                }
            }
        }

    }

}
