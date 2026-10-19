use super::FileManagerApp;
use egui;

impl FileManagerApp {
    pub(crate) fn render_filter_bar(&mut self, ctx: &egui::Context) {
        let visuals = ctx.style().visuals.clone();
        egui::TopBottomPanel::top("filter_bar")
            .frame(
                egui::Frame::default()
                    .fill(visuals.panel_fill)
                    .inner_margin(egui::Margin::symmetric(10.0, 6.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🔍 Filter:").size(13.0));
                    ui.add_space(8.0);
                    
                    if ui.checkbox(&mut self.filter_txt, "📝 TXT").changed() {
                        let _ = self.left_pane.refresh();
                        let _ = self.right_pane.refresh();
                    }
                    ui.add_space(4.0);
                    
                    if ui.checkbox(&mut self.filter_image, "🖼 Image").changed() {
                        let _ = self.left_pane.refresh();
                        let _ = self.right_pane.refresh();
                    }
                    ui.add_space(4.0);
                    
                    if ui.checkbox(&mut self.filter_pdf, "📄 PDF").changed() {
                        let _ = self.left_pane.refresh();
                        let _ = self.right_pane.refresh();
                    }
                    ui.add_space(4.0);
                    
                    if ui.checkbox(&mut self.filter_doc, "📃 DOC").changed() {
                        let _ = self.left_pane.refresh();
                        let _ = self.right_pane.refresh();
                    }
                    ui.add_space(4.0);
                    
                    if ui.checkbox(&mut self.filter_xls, "📊 XLS").changed() {
                        let _ = self.left_pane.refresh();
                        let _ = self.right_pane.refresh();
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("Clear All").clicked() {
                            self.filter_txt = false;
                            self.filter_image = false;
                            self.filter_pdf = false;
                            self.filter_doc = false;
                            self.filter_xls = false;
                            let _ = self.left_pane.refresh();
                            let _ = self.right_pane.refresh();
                        }
                    });
                });
            });

        // Function button bar (before status bar)
    }

    pub(crate) fn render_function_bar_panel(&mut self, ctx: &egui::Context) {
        let visuals = ctx.style().visuals.clone();
        egui::TopBottomPanel::bottom("function_bar")
            .frame(
                egui::Frame::default()
                    .fill(visuals.panel_fill)
                    .inner_margin(egui::Margin::symmetric(8.0, 8.0)),
            )
            .show(ctx, |ui| {
                self.render_function_bar(ui);
            });

        // Bottom status bar - Modern style
    }

    pub(crate) fn render_status_bar(&mut self, ctx: &egui::Context) {
        let visuals = ctx.style().visuals.clone();
        egui::TopBottomPanel::bottom("status_bar")
            .frame(
                egui::Frame::default()
                    .fill(visuals.panel_fill)
                    .inner_margin(egui::Margin::symmetric(10.0, 6.0)),
            )
            .show(ctx, |ui| {
                ui.style_mut()
                    .visuals
                    .widgets
                    .noninteractive
                    .fg_stroke
                    .color = visuals.widgets.noninteractive.fg_stroke.color;
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("📊").size(16.0));
                    ui.add_space(8.0);

                    // Left pane stats
                    let left_stats =
                        crate::filesystem::calculate_directory_stats(&self.left_pane.items);
                    ui.label(
                        egui::RichText::new(format!(
                            "Left: {} items ({} folders, {} files) • {}",
                            left_stats.total_items,
                            left_stats.folder_count,
                            left_stats.file_count,
                            crate::filesystem::format_size(left_stats.total_size)
                        ))
                        .size(14.0),
                    );

                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("│")
                            .size(12.0)
                            .color(visuals.widgets.noninteractive.bg_fill),
                    );
                    ui.add_space(4.0);

                    // Right pane stats
                    let right_stats =
                        crate::filesystem::calculate_directory_stats(&self.right_pane.items);
                    ui.label(
                        egui::RichText::new(format!(
                            "Right: {} items ({} folders, {} files) • {}",
                            right_stats.total_items,
                            right_stats.folder_count,
                            right_stats.file_count,
                            crate::filesystem::format_size(right_stats.total_size)
                        ))
                        .size(14.0),
                    );

                    ui.add_space(8.0);

                    // Git info for active pane
                    let active_pane = self.get_active_pane();
                    if let Some(repo_path) = &active_pane.git_repo_path {
                        if let Ok(git_info) = crate::filesystem::get_git_repo_info(repo_path) {
                            ui.label(
                                egui::RichText::new("│")
                                    .size(12.0)
                                    .color(visuals.widgets.noninteractive.bg_fill),
                            );
                            ui.add_space(4.0);

                            let branch_icon = if git_info.has_changes { "🔶" } else { "🔷" };
                            let mut branch_text =
                                format!("{} {}", branch_icon, git_info.current_branch);

                            if git_info.ahead > 0 || git_info.behind > 0 {
                                branch_text.push_str(&format!(
                                    " (↑{} ↓{})",
                                    git_info.ahead, git_info.behind
                                ));
                            }

                            ui.label(
                                egui::RichText::new(branch_text)
                                    .size(12.0)
                                    .color(visuals.widgets.active.bg_fill),
                            );
                            ui.add_space(8.0);
                        }
                    }

                    // Clipboard indicator
                    if self.clipboard.is_some() {
                        ui.label(
                            egui::RichText::new("│")
                                .size(12.0)
                                .color(visuals.widgets.noninteractive.bg_fill),
                        );
                        ui.add_space(4.0);
                        let op_label = if self
                            .clipboard
                            .as_ref()
                            .map(|(_, op)| matches!(op, super::ClipboardOperation::Cut))
                            .unwrap_or(false)
                        {
                            "✂ Clipboard: cut"
                        } else {
                            "📋 Clipboard: copy"
                        };
                        ui.label(
                            egui::RichText::new(op_label)
                                .size(12.0)
                                .color(visuals.widgets.active.bg_fill),
                        );
                        ui.add_space(8.0);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Multi-selection count (Ctrl/Shift selection feedback
                        // that used to be invisible).
                        let count = self.get_active_pane().get_selected_items().len();
                        if count > 1 {
                            ui.label(
                                egui::RichText::new(format!("☑ {} selected", count))
                                    .size(12.0)
                                    .color(visuals.widgets.active.bg_fill),
                            );
                        }
                    });
                });
            });
    }
}
