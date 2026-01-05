use crate::pane::Pane;
use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
enum ClipboardOperation {
    Copy,
    Cut,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    Dracula,
    Nord,
    Monokai,
    SolarizedDark,
}

impl Theme {
    pub fn name(&self) -> &str {
        match self {
            Theme::Dark => "Dark (Default)",
            Theme::Light => "Light",
            Theme::Dracula => "Dracula",
            Theme::Nord => "Nord",
            Theme::Monokai => "Monokai",
            Theme::SolarizedDark => "Solarized Dark",
        }
    }

    pub fn all() -> Vec<Theme> {
        vec![
            Theme::Dark,
            Theme::Light,
            Theme::Dracula,
            Theme::Nord,
            Theme::Monokai,
            Theme::SolarizedDark,
        ]
    }
}

pub struct ImageViewerState {
    pub image_path: PathBuf,
    pub image_name: String,
    pub texture: Option<egui::TextureHandle>,
    pub zoom: f32,
    pub offset: egui::Vec2,
    pub dragging: bool,
    pub drag_start: egui::Pos2,
}

impl ImageViewerState {
    pub fn new(path: PathBuf, name: String) -> Self {
        Self {
            image_path: path,
            image_name: name,
            texture: None,
            zoom: 1.0,
            offset: egui::Vec2::ZERO,
            dragging: false,
            drag_start: egui::Pos2::ZERO,
        }
    }
}

pub struct FileManagerApp {
    pub left_pane: Pane,
    pub right_pane: Pane,
    pub active_pane: usize,
    pub status_message: String,
    pub show_delete_confirm: bool,
    pub item_to_delete: Option<String>,
    pub show_hidden_files: bool,
    pub clipboard: Option<(std::path::PathBuf, ClipboardOperation)>,
    pub show_context_menu: bool,
    pub context_menu_pos: egui::Pos2,
    pub context_menu_item_index: usize,
    pub context_menu_just_opened: bool,
    pub hovered_item: Option<(usize, usize)>,
    pub filter_mode: bool,
    pub show_new_folder_dialog: bool,
    pub new_folder_name: String,
    pub show_rename_dialog: bool,
    pub rename_new_name: String,
    pub show_about_dialog: bool,
    pub show_properties_dialog: bool,
    pub properties_item: Option<crate::filesystem::FileItem>,
    pub current_theme: Theme,
    pub show_theme_selector: bool,
    pub image_viewer: Option<ImageViewerState>,
    pub show_preview_panel: bool,
    pub preview_content_left: Option<PreviewContent>,
    pub preview_content_right: Option<PreviewContent>,
    pub show_search_dialog: bool,
    pub search_criteria: crate::filesystem::SearchCriteria,
    pub search_results: Vec<crate::filesystem::FileItem>,
    pub search_in_progress: bool,
    pub search_min_size_text: String,
    pub search_max_size_text: String,
    pub search_days_ago: String,
    pub show_compare_dialog: bool,
    pub comparison_result: Option<crate::filesystem::FileComparison>,
    pub compare_scroll_offset: f32,
    pub show_mounts_dialog: bool,
    pub show_sidebar: bool,
    pub bookmark_manager: crate::bookmarks::BookmarkManager,
    pub show_add_bookmark_dialog: bool,
    pub new_bookmark_name: String,
    pub sidebar_quick_access_expanded: bool,
    pub sidebar_bookmarks_expanded: bool,
    pub sidebar_devices_expanded: bool,
    pub filter_txt: bool,
    pub filter_image: bool,
    pub filter_pdf: bool,
    pub filter_doc: bool,
    pub filter_xls: bool,
}

#[derive(Clone)]
pub enum PreviewContent {
    Text(String),
    Image(PathBuf),
    Pdf {
        name: String,
        size: String,
        modified: String,
        pages: usize,
        image: Option<image::DynamicImage>,
    },
    FileInfo {
        name: String,
        size: String,
        modified: String,
        permissions: String,
        is_dir: bool,
    },
}

impl FileManagerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Try to get home directory first, fallback to current dir
        let start_dir = dirs::home_dir()
            .or_else(|| env::current_dir().ok())
            .unwrap_or_else(|| std::path::PathBuf::from("."));

        let left_pane = Pane::new(start_dir.clone()).unwrap_or_else(|e| {
            eprintln!("Error creating left pane: {}", e);
            Pane::new(std::path::PathBuf::from("/")).expect("Failed to create pane")
        });

        let right_pane = Pane::new(start_dir.clone()).unwrap_or_else(|e| {
            eprintln!("Error creating right pane: {}", e);
            Pane::new(std::path::PathBuf::from("/")).expect("Failed to create pane")
        });

        Self {
            left_pane,
            right_pane,
            active_pane: 0,
            status_message: format!("Ready - Starting directory: {}", start_dir.display()),
            show_delete_confirm: false,
            item_to_delete: None,
            show_hidden_files: false,
            clipboard: None,
            show_context_menu: false,
            context_menu_pos: egui::Pos2::ZERO,
            context_menu_item_index: 0,
            context_menu_just_opened: false,
            hovered_item: None,
            filter_mode: false,
            show_new_folder_dialog: false,
            new_folder_name: String::new(),
            show_rename_dialog: false,
            rename_new_name: String::new(),
            show_about_dialog: false,
            show_properties_dialog: false,
            properties_item: None,
            current_theme: Theme::Dark,
            show_theme_selector: false,
            image_viewer: None,
            show_preview_panel: true,
            preview_content_left: None,
            preview_content_right: None,
            show_search_dialog: false,
            search_criteria: crate::filesystem::SearchCriteria::default(),
            search_results: Vec::new(),
            search_in_progress: false,
            search_min_size_text: String::new(),
            search_max_size_text: String::new(),
            search_days_ago: String::new(),
            show_compare_dialog: false,
            comparison_result: None,
            compare_scroll_offset: 0.0,
            show_mounts_dialog: false,
            show_sidebar: true,
            bookmark_manager: crate::bookmarks::BookmarkManager::load().unwrap_or_default(),
            show_add_bookmark_dialog: false,
            new_bookmark_name: String::new(),
            sidebar_quick_access_expanded: true,
            sidebar_bookmarks_expanded: true,
            sidebar_devices_expanded: true,
            filter_txt: false,
            filter_image: false,
            filter_pdf: false,
            filter_doc: false,
            filter_xls: false,
        }
    }

    pub fn get_active_pane(&self) -> &Pane {
        if self.active_pane == 0 {
            &self.left_pane
        } else {
            &self.right_pane
        }
    }

    pub fn get_active_pane_mut(&mut self) -> &mut Pane {
        if self.active_pane == 0 {
            &mut self.left_pane
        } else {
            &mut self.right_pane
        }
    }

    pub fn refresh_pane(&mut self, pane_index: usize) -> Result<()> {
        if pane_index == 0 {
            self.left_pane.refresh()?;
        } else {
            self.right_pane.refresh()?;
        }
        Ok(())
    }

    fn is_image_file(path: &std::path::Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(
                ext.as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp"
            )
        } else {
            false
        }
    }

    pub fn open_image_viewer(&mut self, path: PathBuf, name: String) {
        self.image_viewer = Some(ImageViewerState::new(path, name));
    }

    pub fn close_image_viewer(&mut self) {
        self.image_viewer = None;
    }

    pub fn copy_to_clipboard(&mut self) {
        let item_data = self
            .get_active_pane()
            .get_selected_item()
            .map(|item| (item.path.clone(), item.name.clone()));

        if let Some((path, name)) = item_data {
            if name != ".." {
                self.clipboard = Some((path, ClipboardOperation::Copy));
                self.status_message = format!("Copied to clipboard: {}", name);
            }
        }
    }

    pub fn cut_to_clipboard(&mut self) {
        let item_data = self
            .get_active_pane()
            .get_selected_item()
            .map(|item| (item.path.clone(), item.name.clone()));

        if let Some((path, name)) = item_data {
            if name != ".." {
                self.clipboard = Some((path, ClipboardOperation::Cut));
                self.status_message = format!("Cut to clipboard: {}", name);
            }
        }
    }

    pub fn paste_from_clipboard(&mut self) -> Result<()> {
        // Clone clipboard data to avoid borrow issues
        let clipboard_data = self.clipboard.clone();

        if let Some((source_path, operation)) = clipboard_data {
            let target_path = if self.active_pane == 0 {
                self.right_pane.current_path.clone()
            } else {
                self.left_pane.current_path.clone()
            };

            let file_name = source_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            let dest_path = target_path.join(file_name);

            match operation {
                ClipboardOperation::Copy => {
                    if source_path.is_dir() {
                        self.copy_dir_recursive(&source_path, &dest_path)?;
                        self.status_message =
                            format!("Copied directory: {} → {}", file_name, dest_path.display());
                    } else {
                        fs::copy(&source_path, &dest_path)?;
                        self.status_message =
                            format!("Copied: {} → {}", file_name, dest_path.display());
                    }
                }
                ClipboardOperation::Cut => {
                    fs::rename(&source_path, &dest_path)?;
                    self.status_message = format!("Moved: {} → {}", file_name, dest_path.display());
                    self.clipboard = None; // Clear clipboard after cut
                }
            }

            // Refresh both panes
            self.left_pane.refresh()?;
            self.right_pane.refresh()?;
        } else {
            self.status_message = "Clipboard is empty".to_string();
        }
        Ok(())
    }

    fn copy_dir_recursive(&self, src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                self.copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }

    pub fn open_file_with_default_app(&mut self) -> Result<()> {
        let item_data = self
            .get_active_pane()
            .get_selected_item()
            .map(|item| (item.path.clone(), item.name.clone(), item.is_dir));

        if let Some((path, name, is_dir)) = item_data {
            if !is_dir && name != ".." {
                // Check if it's an image file - open in built-in viewer
                if Self::is_image_file(&path) {
                    self.open_image_viewer(path, name.clone());
                    self.status_message = format!("Opening image: {}", name);
                    return Ok(());
                }

                // Use system default app to open file
                #[cfg(target_os = "linux")]
                {
                    std::process::Command::new("xdg-open").arg(&path).spawn()?;
                    self.status_message = format!("Opening: {}", name);
                }

                #[cfg(target_os = "macos")]
                {
                    std::process::Command::new("open").arg(&path).spawn()?;
                    self.status_message = format!("Opening: {}", name);
                }

                #[cfg(target_os = "windows")]
                {
                    std::process::Command::new("cmd")
                        .args(["/C", "start", "", &path.to_string_lossy()])
                        .spawn()?;
                    self.status_message = format!("Opening: {}", name);
                }
            } else if is_dir && name != ".." {
                self.status_message = "Use Enter or double-click to open directories".to_string();
            }
        }
        Ok(())
    }

    pub fn delete_selected_file(&mut self) -> Result<()> {
        let item = self.get_active_pane().get_selected_item().cloned();

        if let Some(item) = item {
            if item.name != ".." {
                let msg = if item.is_dir {
                    fs::remove_dir_all(&item.path)?;
                    format!("Deleted directory: {}", item.name)
                } else {
                    fs::remove_file(&item.path)?;
                    format!("Deleted file: {}", item.name)
                };
                self.status_message = msg;
                self.get_active_pane_mut().refresh()?;
            }
        }
        Ok(())
    }

    pub fn compress_item(&mut self) -> Result<()> {
        let item = self.get_active_pane().get_selected_item().cloned();

        if let Some(item) = item {
            if item.name != ".." {
                let zip_name = format!("{}.zip", item.name);
                let zip_path = item
                    .path
                    .parent()
                    .unwrap_or(item.path.as_path())
                    .join(&zip_name);

                match crate::filesystem::compress_to_zip(&item.path, &zip_path) {
                    Ok(_) => {
                        self.status_message =
                            format!("✅ Compressed: {} → {}", item.name, zip_name);
                        self.get_active_pane_mut().refresh()?;
                    }
                    Err(e) => {
                        self.status_message = format!("❌ Compression failed: {}", e);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn extract_archive(&mut self) -> Result<()> {
        let item = self.get_active_pane().get_selected_item().cloned();

        if let Some(item) = item {
            if item.name != ".." && !item.is_dir {
                let extract_dir = item.path.parent().unwrap_or(item.path.as_path());

                if item.name.ends_with(".zip") {
                    match crate::filesystem::extract_zip(&item.path, extract_dir) {
                        Ok(_) => {
                            self.status_message = format!("✅ Extracted: {}", item.name);
                            self.get_active_pane_mut().refresh()?;
                        }
                        Err(e) => {
                            self.status_message = format!("❌ Extraction failed: {}", e);
                        }
                    }
                } else {
                    self.status_message = "Only .zip files are supported".to_string();
                }
            }
        }
        Ok(())
    }

    pub fn compare_selected_files(&mut self) -> Result<()> {
        let left_item = self.left_pane.get_selected_item().cloned();
        let right_item = self.right_pane.get_selected_item().cloned();

        if let (Some(left), Some(right)) = (left_item, right_item) {
            if left.name == ".." || right.name == ".." {
                self.status_message = "Cannot compare parent directory".to_string();
                return Ok(());
            }

            if left.is_dir || right.is_dir {
                self.status_message = "Cannot compare directories (files only)".to_string();
                return Ok(());
            }

            match crate::filesystem::compare_files(&left.path, &right.path) {
                Ok(comparison) => {
                    self.comparison_result = Some(comparison.clone());
                    self.show_compare_dialog = true;
                    self.compare_scroll_offset = 0.0;

                    if comparison.are_identical {
                        self.status_message = "✅ Files are identical".to_string();
                    } else {
                        self.status_message = format!(
                            "Files differ: {} added, {} removed, {} modified",
                            comparison.right_only_lines,
                            comparison.left_only_lines,
                            comparison.modified_lines
                        );
                    }
                }
                Err(e) => {
                    self.status_message = format!("❌ Comparison failed: {}", e);
                }
            }
        } else {
            self.status_message = "Select files in both panes to compare".to_string();
        }

        Ok(())
    }

    pub fn should_show_file(&self, item: &crate::filesystem::FileItem) -> bool {
        if !self.filter_txt && !self.filter_image && !self.filter_pdf && !self.filter_doc && !self.filter_xls {
            return true;
        }

        if item.is_dir {
            return true;
        }

        if let Some(ext) = item.path.extension() {
            let ext_lower = ext.to_string_lossy().to_lowercase();
            
            if self.filter_txt && matches!(ext_lower.as_str(), "txt" | "log" | "md" | "json" | "xml" | "csv") {
                return true;
            }
            if self.filter_image && matches!(ext_lower.as_str(), "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp" | "svg") {
                return true;
            }
            if self.filter_pdf && ext_lower == "pdf" {
                return true;
            }
            if self.filter_doc && matches!(ext_lower.as_str(), "doc" | "docx" | "odt" | "rtf") {
                return true;
            }
            if self.filter_xls && matches!(ext_lower.as_str(), "xls" | "xlsx" | "ods" | "csv") {
                return true;
            }
        }

        false
    }
}

impl eframe::App for FileManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply modern visual style
        ctx.set_visuals(self.get_modern_visuals());
        ctx.set_zoom_factor(1.1);
        let visuals = ctx.style().visuals.clone();

        // Top menu bar - Modern style
        egui::TopBottomPanel::top("menu_bar")
            .frame(
                egui::Frame::default()
                    .fill(visuals.panel_fill)
                    .inner_margin(egui::Margin::symmetric(8.0, 6.0)),
            )
            .show(ctx, |ui| {
                ui.style_mut().visuals.widgets.inactive.fg_stroke.color =
                    visuals.widgets.inactive.fg_stroke.color;
                ui.style_mut().visuals.widgets.hovered.fg_stroke.color =
                    visuals.widgets.hovered.fg_stroke.color;

                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Files", |ui| {
                        if ui.button("Refresh (F2)").clicked() {
                            let _ = self.left_pane.refresh();
                            let _ = self.right_pane.refresh();
                            self.status_message = "Refreshed".to_string();
                            ui.close_menu();
                        }

                        ui.separator();

                        // Toggle hidden files
                        let hidden_text = if self.show_hidden_files {
                            "☑ Show Hidden Files (Ctrl+H)"
                        } else {
                            "☐ Show Hidden Files (Ctrl+H)"
                        };
                        if ui.button(hidden_text).clicked() {
                            self.show_hidden_files = !self.show_hidden_files;
                            let _ = self.left_pane.refresh();
                            let _ = self.right_pane.refresh();
                            self.status_message = if self.show_hidden_files {
                                "Showing hidden files".to_string()
                            } else {
                                "Hiding hidden files".to_string()
                            };
                            ui.close_menu();
                        }

                        // Toggle preview panel
                        let preview_text = if self.show_preview_panel {
                            "☑ Show Preview Panel (Space)"
                        } else {
                            "☐ Show Preview Panel (Space)"
                        };
                        if ui.button(preview_text).clicked() {
                            self.show_preview_panel = !self.show_preview_panel;
                            ui.close_menu();
                        }

                        let sidebar_text = if self.show_sidebar {
                            "☑ Show Sidebar (Ctrl+B)"
                        } else {
                            "☐ Show Sidebar (Ctrl+B)"
                        };
                        if ui.button(sidebar_text).clicked() {
                            self.show_sidebar = !self.show_sidebar;
                            ui.close_menu();
                        }

                        ui.separator();

                        // Theme selector
                        if ui.button("🎨 Themes...").clicked() {
                            self.show_theme_selector = true;
                            ui.close_menu();
                        }

                        ui.separator();
                        if ui.button("Exit (Alt+F4)").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });

                    ui.menu_button("Bookmarks", |ui| {
                        if ui.button("⭐ Add Current Folder").clicked() {
                            self.show_add_bookmark_dialog = true;
                            self.new_bookmark_name = self
                                .get_active_pane()
                                .current_path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Bookmark")
                                .to_string();
                            ui.close_menu();
                        }

                        ui.separator();

                        let bookmarks = self.bookmark_manager.get_bookmarks().to_vec();
                        if bookmarks.is_empty() {
                            ui.label("No bookmarks yet");
                        } else {
                            for (idx, bookmark) in bookmarks.iter().enumerate() {
                                if ui
                                    .button(format!("{} {}", bookmark.icon, bookmark.name))
                                    .clicked()
                                {
                                    let path = bookmark.path.clone();
                                    let _ = self.get_active_pane_mut().navigate_to(path.clone());
                                    self.status_message =
                                        format!("Navigated to {}", path.display());
                                    ui.close_menu();
                                }
                            }
                        }
                    });

                    ui.menu_button("Commands", |ui| {
                        if ui.button("💾 Mount Points").clicked() {
                            self.show_mounts_dialog = true;
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("📁 New Folder (Ctrl+N)").clicked() {
                            self.show_new_folder_dialog = true;
                            ui.close_menu();
                        }
                        if ui.button("✏ Rename (F2)").clicked() {
                            if let Some(item) = self.get_active_pane().get_selected_item() {
                                if item.name != ".." {
                                    self.rename_new_name = item.name.clone();
                                    self.show_rename_dialog = true;
                                }
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("📄 Open File (F3)").clicked() {
                            let _ = self.open_file_with_default_app();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("🔍 Advanced Search (Ctrl+Shift+F)").clicked() {
                            self.search_criteria.search_path =
                                self.get_active_pane().current_path.clone();
                            self.show_search_dialog = true;
                            ui.close_menu();
                        }
                        if ui.button("⚖ Compare Files (Ctrl+D)").clicked() {
                            let _ = self.compare_selected_files();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("📋 Copy (F5)").clicked() {
                            self.copy_to_clipboard();
                            ui.close_menu();
                        }
                        if ui.button("✂ Cut/Move (F6)").clicked() {
                            self.cut_to_clipboard();
                            ui.close_menu();
                        }
                        if ui.button("📎 Paste (F7)").clicked() {
                            let _ = self.paste_from_clipboard();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("🗑 Delete (F8)").clicked() {
                            if let Some(item) = self.get_active_pane().get_selected_item() {
                                self.item_to_delete = Some(item.name.clone());
                                self.show_delete_confirm = true;
                            }
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Help", |ui| {
                        if ui.button("⚡ About").clicked() {
                            self.show_about_dialog = true;
                            ui.close_menu();
                        }
                        if ui.button("💝 Donate").clicked() {
                        if let Err(e) = open::that("https://www.paypal.com/paypalme/AchmadFachrie") {
                        self.status_message = format!("Failed to open PayPal: {}", e);
                        } else {
                        self.status_message = "Opening PayPal donation page...".to_string();
                        }
                            ui.close_menu();
                        }
                        ui.separator();
                        ui.label("Keyboard Shortcuts:");
                        ui.label("• F2: Refresh");
                        ui.label("• F3: Open file");
                        ui.label("• F5: Copy");
                        ui.label("• F6: Cut/Move");
                        ui.label("• F7: Paste");
                        ui.label("• F8: Delete");
                        ui.label("• Space: Toggle preview");
                        ui.label("• Ctrl+H: Toggle hidden");
                        ui.label("• Ctrl+F: Filter");
                        ui.label("• Tab: Switch pane");
                        ui.label("• Alt+←/→: Back/Forward");
                    });
                });
            });

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
        egui::TopBottomPanel::bottom("function_bar")
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(45, 47, 50))
                    .inner_margin(egui::Margin::symmetric(8.0, 8.0)),
            )
            .show(ctx, |ui| {
                self.render_function_bar(ui);
            });

        // Bottom status bar - Modern style
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
                        ui.label(
                            egui::RichText::new("📋 Clipboard ready")
                                .size(12.0)
                                .color(visuals.widgets.active.bg_fill),
                        );
                        ui.add_space(8.0);
                    }

                    // Status message
                    if !self.status_message.is_empty() {
                        ui.label(
                            egui::RichText::new("│")
                                .size(12.0)
                                .color(visuals.widgets.noninteractive.bg_fill),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(&self.status_message)
                                .size(12.0)
                                .color(visuals.widgets.noninteractive.fg_stroke.color),
                        );
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("⚡ Ready")
                                .size(12.0)
                                .color(visuals.widgets.active.bg_fill),
                        );
                    });
                });
            });

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

        // Main dual-pane area - fills remaining space
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(visuals.panel_fill)
                    .inner_margin(2.0),
            )
            .show(ctx, |ui| {
                let full_rect = ui.available_rect_before_wrap();
                let preview_height = if self.show_preview_panel { 200.0 } else { 0.0 };
                let spacing = if self.show_preview_panel { 4.0 } else { 0.0 };
                
                // Use StripBuilder for proper responsive layout with exact sizes
                egui_extras::StripBuilder::new(ui)
                    .size(egui_extras::Size::remainder()) // Panes area
                    .size(egui_extras::Size::exact(preview_height + spacing)) // Preview
                    .vertical(|mut strip| {
                        // Panes strip - horizontal layout
                        strip.cell(|ui| {
                            // Calculate available width and split evenly between panes
                            let available_width = ui.available_width();
                            let transfer_btn_width = 50.0;
                            let pane_width = (available_width - transfer_btn_width) / 2.0;
                            
                            egui_extras::StripBuilder::new(ui)
                                .size(egui_extras::Size::exact(pane_width)) // Left pane - exact half
                                .size(egui_extras::Size::exact(transfer_btn_width)) // Transfer buttons - fixed width
                                .size(egui_extras::Size::exact(pane_width)) // Right pane - exact half
                                .horizontal(|mut strip| {
                                    // Left pane
                                    strip.cell(|ui| {
                                        ui.set_clip_rect(ui.max_rect());
                                        self.render_pane(ui, 0, ctx, pane_width);
                                    });
                                    
                                    // Transfer buttons
                                    strip.cell(|ui| {
                                        self.render_transfer_buttons(ui);
                                    });
                                    
                                    // Right pane
                                    strip.cell(|ui| {
                                        ui.set_clip_rect(ui.max_rect());
                                        self.render_pane(ui, 1, ctx, pane_width);
                                    });
                                });
                        });
                        
                        // Preview panel
                        if self.show_preview_panel {
                            strip.cell(|ui| {
                                self.render_preview_panel(ui, ctx);
                            });
                        }
                    });
            });

        // Handle keyboard shortcuts
        self.handle_keyboard(ctx);

        // Image Viewer Window
        if let Some(viewer) = &mut self.image_viewer {
            let mut close_viewer = false;

            egui::Window::new(format!("🖼 {}", viewer.image_name))
                .collapsible(false)
                .resizable(true)
                .default_size([800.0, 600.0])
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    // Load image if not loaded
                    if viewer.texture.is_none() {
                        match image::open(&viewer.image_path) {
                            Ok(img) => {
                                let size = [img.width() as usize, img.height() as usize];
                                let rgba = img.to_rgba8();
                                let pixels = rgba.as_flat_samples();

                                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                    size,
                                    pixels.as_slice(),
                                );

                                viewer.texture = Some(ctx.load_texture(
                                    &viewer.image_name,
                                    color_image,
                                    egui::TextureOptions::LINEAR,
                                ));
                            }
                            Err(e) => {
                                ui.label(
                                    egui::RichText::new(format!("Error loading image: {}", e))
                                        .color(egui::Color32::RED)
                                        .size(14.0),
                                );
                                if ui.button("Close").clicked() {
                                    close_viewer = true;
                                }
                                return;
                            }
                        }
                    }

                    // Toolbar
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Zoom:").size(12.0));
                        if ui.button("➖").clicked() {
                            viewer.zoom = (viewer.zoom * 0.8).max(0.1);
                        }
                        ui.label(
                            egui::RichText::new(format!("{:.0}%", viewer.zoom * 100.0)).size(12.0),
                        );
                        if ui.button("➕").clicked() {
                            viewer.zoom = (viewer.zoom * 1.2).min(10.0);
                        }
                        if ui.button("🔄 Reset").clicked() {
                            viewer.zoom = 1.0;
                            viewer.offset = egui::Vec2::ZERO;
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("✗ Close (ESC)").clicked() {
                                close_viewer = true;
                            }
                        });
                    });

                    ui.separator();

                    // Image display area
                    if let Some(texture) = &viewer.texture {
                        let available_size = ui.available_size();
                        let img_size = texture.size_vec2();
                        let scaled_size = img_size * viewer.zoom;

                        // Scrollable area for panning
                        egui::ScrollArea::both()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                let (rect, response) = ui.allocate_exact_size(
                                    scaled_size,
                                    egui::Sense::click_and_drag(),
                                );

                                // Handle dragging
                                if response.dragged() {
                                    viewer.offset += response.drag_delta();
                                }

                                // Handle scroll wheel zoom
                                if response.hovered() {
                                    let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
                                    if scroll_delta != 0.0 {
                                        let zoom_delta = 1.0 + scroll_delta * 0.001;
                                        viewer.zoom = (viewer.zoom * zoom_delta).clamp(0.1, 10.0);
                                    }
                                }

                                // Draw image
                                ui.painter().image(
                                    texture.id(),
                                    rect,
                                    egui::Rect::from_min_max(
                                        egui::pos2(0.0, 0.0),
                                        egui::pos2(1.0, 1.0),
                                    ),
                                    egui::Color32::WHITE,
                                );

                                // Show image info
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Size: {}x{} | Zoom: {:.0}%",
                                        img_size.x,
                                        img_size.y,
                                        viewer.zoom * 100.0
                                    ))
                                    .size(11.0)
                                    .color(egui::Color32::GRAY),
                                );
                            });
                    }
                });

            // Handle ESC key to close
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                close_viewer = true;
            }

            if close_viewer {
                self.close_image_viewer();
            }
        }

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
                                        let _ = self.compress_item();
                                        close_menu = true;
                                    }

                                    // Extract (only for .zip files)
                                    if !item.is_dir && item.name.ends_with(".zip") {
                                        if ui
                                            .add_sized(
                                                [ui.available_width(), 28.0],
                                                egui::Button::new(
                                                    egui::RichText::new("📦  Extract ZIP")
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

        // Properties Dialog
        if self.show_properties_dialog {
            if let Some(item) = self.properties_item.clone() {
                egui::Window::new("ℹ Properties")
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

                            let (icon, icon_color) = if item.is_dir {
                                ("📁", egui::Color32::from_rgb(255, 184, 108))
                            } else {
                                ("📄", egui::Color32::from_rgb(189, 193, 198))
                            };

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(icon).size(32.0).color(icon_color));
                                ui.add_space(10.0);
                                ui.label(egui::RichText::new(&item.name).size(16.0).strong());
                            });

                            ui.add_space(15.0);
                            ui.separator();
                            ui.add_space(10.0);

                            egui::Grid::new("properties_grid")
                                .num_columns(2)
                                .spacing([20.0, 8.0])
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new("Type:").strong());
                                    ui.label(if item.is_dir { "Directory" } else { "File" });
                                    ui.end_row();

                                    ui.label(egui::RichText::new("Size:").strong());
                                    ui.label(crate::filesystem::format_size(item.size));
                                    ui.end_row();

                                    ui.label(egui::RichText::new("Modified:").strong());
                                    ui.label(crate::filesystem::format_date(item.modified));
                                    ui.end_row();

                                    // Permissions
                                    ui.label(egui::RichText::new("Permissions:").strong());
                                    match crate::filesystem::get_permissions(&item.path) {
                                        Ok((summary, details)) => {
                                            ui.vertical(|ui| {
                                                ui.label(
                                                    egui::RichText::new(summary)
                                                        .family(egui::FontFamily::Monospace)
                                                        .color(egui::Color32::from_rgb(
                                                            138, 180, 248,
                                                        )),
                                                );
                                                ui.add_space(5.0);
                                                ui.label(
                                                    egui::RichText::new(details).size(11.0).color(
                                                        egui::Color32::from_rgb(189, 193, 198),
                                                    ),
                                                );
                                            });
                                        }
                                        Err(_) => {
                                            ui.label(
                                                egui::RichText::new("N/A")
                                                    .color(egui::Color32::GRAY),
                                            );
                                        }
                                    }
                                    ui.end_row();

                                    ui.label(egui::RichText::new("Path:").strong());
                                    ui.label(item.path.display().to_string());
                                    ui.end_row();
                                });

                            ui.add_space(15.0);
                            ui.separator();
                            ui.add_space(10.0);

                            ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() / 2.0 - 100.0);
                                if ui
                                    .add_sized(
                                        [200.0, 36.0],
                                        egui::Button::new(egui::RichText::new("Close").size(14.0))
                                            .fill(egui::Color32::from_rgb(66, 133, 244))
                                            .rounding(6.0),
                                    )
                                    .clicked()
                                {
                                    self.show_properties_dialog = false;
                                    self.properties_item = None;
                                }
                            });
                        });
                    });
            }
        }

        // New Folder Dialog
        if self.show_new_folder_dialog {
            egui::Window::new("📁 Create New Folder")
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
                        ui.label(egui::RichText::new("Folder name:").size(14.0));
                        ui.add_space(5.0);
                        let response = ui.add_sized(
                            [300.0, 30.0],
                            egui::TextEdit::singleline(&mut self.new_folder_name)
                                .hint_text("Enter folder name..."),
                        );
                        response.request_focus();

                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✓ Create").size(14.0))
                                        .fill(egui::Color32::from_rgb(40, 167, 69))
                                        .rounding(6.0),
                                )
                                .clicked()
                                || (response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                            {
                                if !self.new_folder_name.is_empty() {
                                    let new_path = self
                                        .get_active_pane()
                                        .current_path
                                        .join(&self.new_folder_name);
                                    match fs::create_dir(&new_path) {
                                        Ok(_) => {
                                            self.status_message =
                                                format!("Created folder: {}", self.new_folder_name);
                                            let _ = self.get_active_pane_mut().refresh();
                                        }
                                        Err(e) => {
                                            self.status_message =
                                                format!("Error creating folder: {}", e);
                                        }
                                    }
                                    self.new_folder_name.clear();
                                    self.show_new_folder_dialog = false;
                                }
                            }
                            ui.add_space(10.0);
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✗ Cancel").size(14.0))
                                        .fill(egui::Color32::from_rgb(66, 70, 77))
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                self.new_folder_name.clear();
                                self.show_new_folder_dialog = false;
                            }
                        });
                    });
                });
        }

        // Rename Dialog
        if self.show_rename_dialog {
            egui::Window::new("✏ Rename")
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
                        ui.label(egui::RichText::new("New name:").size(14.0));
                        ui.add_space(5.0);
                        let response = ui.add_sized(
                            [300.0, 30.0],
                            egui::TextEdit::singleline(&mut self.rename_new_name),
                        );
                        response.request_focus();

                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✓ Rename").size(14.0))
                                        .fill(egui::Color32::from_rgb(40, 167, 69))
                                        .rounding(6.0),
                                )
                                .clicked()
                                || (response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                            {
                                if !self.rename_new_name.is_empty() {
                                    if let Some(item) = self.get_active_pane().get_selected_item() {
                                        let old_path = item.path.clone();
                                        let new_path = item
                                            .path
                                            .parent()
                                            .unwrap_or(item.path.as_ref())
                                            .join(&self.rename_new_name);
                                        match fs::rename(&old_path, &new_path) {
                                            Ok(_) => {
                                                self.status_message = format!(
                                                    "Renamed: {} → {}",
                                                    item.name, self.rename_new_name
                                                );
                                                let _ = self.get_active_pane_mut().refresh();
                                            }
                                            Err(e) => {
                                                self.status_message =
                                                    format!("Error renaming: {}", e);
                                            }
                                        }
                                    }
                                    self.rename_new_name.clear();
                                    self.show_rename_dialog = false;
                                }
                            }
                            ui.add_space(10.0);
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✗ Cancel").size(14.0))
                                        .fill(egui::Color32::from_rgb(66, 70, 77))
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                self.rename_new_name.clear();
                                self.show_rename_dialog = false;
                            }
                        });
                    });
                });
        }

        // About Dialog
        if self.show_about_dialog {
            egui::Window::new("About")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(egui::Color32::from_rgb(45, 47, 50))
                        .rounding(12.0)
                        .inner_margin(25.0),
                )
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("⚡").size(64.0));
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new("Filane - Dual Pane FM")
                                .size(20.0)
                                .strong(),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION")))
                                .size(12.0)
                                .color(visuals.widgets.noninteractive.fg_stroke.color),
                        );
                        ui.add_space(15.0);
                        ui.label(egui::RichText::new("A modern, fast file manager").size(13.0));
                        ui.label(egui::RichText::new("built with Rust and egui").size(13.0));
                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new("Developed by AI Fach")
                                .size(13.0)
                                .color(visuals.widgets.active.bg_fill),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new("💝 Support this project")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(255, 121, 198)),
                        );
                        ui.add_space(5.0);
                        if ui
                            .add_sized(
                                [200.0, 36.0],
                                egui::Button::new(egui::RichText::new("☕ Donate via PayPal").size(13.0))
                                    .fill(egui::Color32::from_rgb(0, 112, 186))
                                    .rounding(6.0),
                            )
                            .clicked()
                        {
                            if let Err(e) = open::that("https://www.paypal.com/paypalme/AchmadFachrie") {
                                self.status_message = format!("Failed to open PayPal: {}", e);
                            } else {
                                self.status_message = "Opening PayPal donation page...".to_string();
                            }
                        }
                        ui.add_space(20.0);
                        if ui
                            .add_sized(
                                [200.0, 36.0],
                                egui::Button::new(egui::RichText::new("Close").size(14.0))
                                    .fill(egui::Color32::from_rgb(66, 133, 244))
                                    .rounding(6.0),
                            )
                            .clicked()
                        {
                            self.show_about_dialog = false;
                        }
                        ui.add_space(10.0);
                    });
                });
        }

        // Theme Selector Dialog
        if self.show_theme_selector {
            let visuals = ctx.style().visuals.clone();
            
            egui::Window::new("🎨 Theme Selector")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(visuals.panel_fill)
                        .rounding(12.0)
                        .inner_margin(20.0),
                )
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new("Choose a theme:").size(14.0).strong());
                        ui.add_space(10.0);

                        for theme in Theme::all() {
                            let is_current = self.current_theme == theme;
                            let button_text = if is_current {
                                format!("✓ {}", theme.name())
                            } else {
                                format!("   {}", theme.name())
                            };

                            let button = ui.add_sized(
                                [280.0, 36.0],
                                egui::Button::new(egui::RichText::new(button_text).size(14.0))
                                    .fill(if is_current {
                                        visuals.widgets.active.bg_fill
                                    } else {
                                        visuals.widgets.inactive.bg_fill
                                    })
                                    .rounding(6.0),
                            );

                            if button.clicked() {
                                self.current_theme = theme;
                                self.status_message = format!("Theme changed to: {}", theme.name());
                            }

                            ui.add_space(6.0);
                        }

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            ui.add_space(ui.available_width() / 2.0 - 100.0);
                            if ui
                                .add_sized(
                                    [200.0, 36.0],
                                    egui::Button::new(egui::RichText::new("Close").size(14.0))
                                        .fill(visuals.widgets.inactive.bg_fill)
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                self.show_theme_selector = false;
                            }
                        });
                    });
                });
        }

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
                            let mut navigate_to: Option<PathBuf> = None;

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
                                            if let Some(parent) = item.path.parent() {
                                                navigate_to = Some(parent.to_path_buf());
                                            }
                                        }
                                    }
                                });

                            if let Some(path) = navigate_to {
                                let _ = self.get_active_pane_mut().navigate_to(path);
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

        // Mount Points Dialog
        if self.show_mounts_dialog {
            let mut close_dialog = false;

            egui::Window::new("💾 Mount Points")
                .collapsible(false)
                .resizable(true)
                .default_size([800.0, 500.0])
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

                        ui.label(
                            egui::RichText::new("Available mount points and storage devices")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(189, 193, 198)),
                        );

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        let mounts = crate::filesystem::get_mount_points();

                        egui::ScrollArea::vertical()
                            .max_height(350.0)
                            .show(ui, |ui| {
                                for mount in &mounts {
                                    let icon = if mount.is_removable { "🔌" } else { "💾" };
                                    let usage = mount.usage_percentage();
                                    let used = mount.total_space - mount.available_space;

                                    let usage_color = if usage > 90.0 {
                                        egui::Color32::from_rgb(242, 139, 130)
                                    } else if usage > 70.0 {
                                        egui::Color32::from_rgb(255, 193, 7)
                                    } else {
                                        egui::Color32::from_rgb(129, 201, 149)
                                    };

                                    egui::Frame::default()
                                        .fill(egui::Color32::from_rgb(50, 52, 55))
                                        .rounding(8.0)
                                        .inner_margin(12.0)
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new(icon).size(24.0));
                                                ui.add_space(8.0);

                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            egui::RichText::new(
                                                                mount
                                                                    .mount_point
                                                                    .display()
                                                                    .to_string(),
                                                            )
                                                            .size(14.0)
                                                            .strong()
                                                            .color(egui::Color32::from_rgb(
                                                                138, 180, 248,
                                                            )),
                                                        );

                                                        if mount.is_removable {
                                                            ui.label(
                                                                egui::RichText::new("Removable")
                                                                    .size(10.0)
                                                                    .color(
                                                                        egui::Color32::from_rgb(
                                                                            189, 147, 249,
                                                                        ),
                                                                    ),
                                                            );
                                                        }
                                                    });

                                                    ui.add_space(4.0);

                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "Device: {}",
                                                                mount.device_name
                                                            ))
                                                            .size(11.0)
                                                            .color(egui::Color32::from_rgb(
                                                                189, 193, 198,
                                                            )),
                                                        );
                                                        ui.label(
                                                            egui::RichText::new("•").size(11.0),
                                                        );
                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "Type: {}",
                                                                mount.file_system
                                                            ))
                                                            .size(11.0)
                                                            .color(egui::Color32::from_rgb(
                                                                189, 193, 198,
                                                            )),
                                                        );
                                                        ui.label(
                                                            egui::RichText::new("•").size(11.0),
                                                        );
                                                        ui.label(
                                                            egui::RichText::new(&mount.disk_kind)
                                                                .size(11.0)
                                                                .color(egui::Color32::from_rgb(
                                                                    189, 193, 198,
                                                                )),
                                                        );
                                                    });

                                                    ui.add_space(6.0);

                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "{} / {}",
                                                                crate::filesystem::format_size(
                                                                    used
                                                                ),
                                                                crate::filesystem::format_size(
                                                                    mount.total_space
                                                                )
                                                            ))
                                                            .size(11.0),
                                                        );

                                                        ui.label(
                                                            egui::RichText::new("•").size(11.0),
                                                        );

                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "Available: {}",
                                                                crate::filesystem::format_size(
                                                                    mount.available_space
                                                                )
                                                            ))
                                                            .size(11.0)
                                                            .color(egui::Color32::from_rgb(
                                                                129, 201, 149,
                                                            )),
                                                        );
                                                    });

                                                    ui.add_space(6.0);

                                                    let progress_width =
                                                        ui.available_width() - 80.0;
                                                    ui.horizontal(|ui| {
                                                        let progress_bar =
                                                            egui::ProgressBar::new(usage / 100.0)
                                                                .desired_width(progress_width)
                                                                .fill(usage_color);
                                                        ui.add(progress_bar);

                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "{:.1}%",
                                                                usage
                                                            ))
                                                            .size(11.0)
                                                            .strong()
                                                            .color(usage_color),
                                                        );
                                                    });

                                                    ui.add_space(6.0);

                                                    if ui
                                                        .button(
                                                            egui::RichText::new("📂 Open")
                                                                .size(11.0),
                                                        )
                                                        .clicked()
                                                    {
                                                        let active_pane =
                                                            self.get_active_pane_mut();
                                                        if let Ok(()) = active_pane
                                                            .navigate_to(mount.mount_point.clone())
                                                        {
                                                            self.status_message = format!(
                                                                "Navigated to {}",
                                                                mount.mount_point.display()
                                                            );
                                                            close_dialog = true;
                                                        }
                                                    }
                                                });
                                            });
                                        });

                                    ui.add_space(8.0);
                                }

                                if mounts.is_empty() {
                                    ui.vertical_centered(|ui| {
                                        ui.add_space(40.0);
                                        ui.label(
                                            egui::RichText::new("No mount points found")
                                                .size(14.0)
                                                .color(egui::Color32::from_rgb(154, 160, 166)),
                                        );
                                    });
                                }
                            });

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .add_sized(
                                            [100.0, 32.0],
                                            egui::Button::new(
                                                egui::RichText::new("Close").size(12.0),
                                            )
                                            .fill(egui::Color32::from_rgb(66, 70, 77))
                                            .rounding(6.0),
                                        )
                                        .clicked()
                                    {
                                        close_dialog = true;
                                    }
                                },
                            );
                        });
                    });
                });

            if close_dialog {
                self.show_mounts_dialog = false;
            }
        }

        // Add Bookmark Dialog
        if self.show_add_bookmark_dialog {
            egui::Window::new("⭐ Add Bookmark")
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

                        ui.label(egui::RichText::new("Bookmark name:").size(14.0));
                        ui.add_space(5.0);
                        let response = ui.add_sized(
                            [300.0, 30.0],
                            egui::TextEdit::singleline(&mut self.new_bookmark_name)
                                .hint_text("Enter bookmark name..."),
                        );
                        response.request_focus();

                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "Path: {}",
                                self.get_active_pane().current_path.display()
                            ))
                            .size(11.0)
                            .color(egui::Color32::from_rgb(138, 180, 248)),
                        );

                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✓ Add").size(14.0))
                                        .fill(egui::Color32::from_rgb(40, 167, 69))
                                        .rounding(6.0),
                                )
                                .clicked()
                                || (response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                            {
                                if !self.new_bookmark_name.is_empty() {
                                    let path = self.get_active_pane().current_path.clone();
                                    match self.bookmark_manager.add_bookmark(
                                        self.new_bookmark_name.clone(),
                                        path.clone(),
                                        "📁".to_string(),
                                    ) {
                                        Ok(_) => {
                                            self.status_message = format!(
                                                "Added bookmark: {}",
                                                self.new_bookmark_name
                                            );
                                        }
                                        Err(e) => {
                                            self.status_message =
                                                format!("Error adding bookmark: {}", e);
                                        }
                                    }
                                    self.new_bookmark_name.clear();
                                    self.show_add_bookmark_dialog = false;
                                }
                            }
                            ui.add_space(10.0);
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✗ Cancel").size(14.0))
                                        .fill(egui::Color32::from_rgb(66, 70, 77))
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                self.new_bookmark_name.clear();
                                self.show_add_bookmark_dialog = false;
                            }
                        });
                    });
                });
        }

        // Delete confirmation dialog
        if self.show_delete_confirm {
            let name = self.item_to_delete.clone().unwrap_or_default();
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
                        ui.label(egui::RichText::new("Delete this item?").size(16.0).strong());
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(&name)
                                .size(14.0)
                                .color(egui::Color32::from_rgb(138, 180, 248)),
                        );
                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            if ui
                                .add_sized(
                                    [120.0, 36.0],
                                    egui::Button::new(
                                        egui::RichText::new("✓  Yes, Delete").size(14.0),
                                    )
                                    .fill(egui::Color32::from_rgb(220, 53, 69))
                                    .rounding(6.0),
                                )
                                .clicked()
                            {
                                let _ = self.delete_selected_file();
                                self.show_delete_confirm = false;
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
                                self.item_to_delete = None;
                            }
                        });
                        ui.add_space(10.0);
                    });
                });
        }
        
    }
}

impl FileManagerApp {
    fn get_modern_visuals(&self) -> egui::Visuals {
        match self.current_theme {
            Theme::Dark => self.get_dark_theme(),
            Theme::Light => self.get_light_theme(),
            Theme::Dracula => self.get_dracula_theme(),
            Theme::Nord => self.get_nord_theme(),
            Theme::Monokai => self.get_monokai_theme(),
            Theme::SolarizedDark => self.get_solarized_dark_theme(),
        }
    }

    fn get_dark_theme(&self) -> egui::Visuals {
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

    fn get_light_theme(&self) -> egui::Visuals {
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

    fn get_dracula_theme(&self) -> egui::Visuals {
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

    fn get_nord_theme(&self) -> egui::Visuals {
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

    fn get_monokai_theme(&self) -> egui::Visuals {
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

    fn get_solarized_dark_theme(&self) -> egui::Visuals {
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

    fn render_function_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.style_mut().spacing.button_padding = egui::vec2(14.0, 10.0);
            ui.style_mut().visuals.widgets.inactive.fg_stroke.color =
                egui::Color32::from_rgb(232, 234, 237);
            ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                egui::Color32::from_rgb(66, 70, 77);

            // Modern function buttons with tooltips
            if ui
                .add_sized(
                    [95.0, 38.0],
                    egui::Button::new(egui::RichText::new("💾 F2").size(13.0))
                        .fill(egui::Color32::from_rgb(55, 58, 64))
                        .rounding(6.0),
                )
                .on_hover_text("Refresh both panes")
                .clicked()
            {
                let _ = self.left_pane.refresh();
                let _ = self.right_pane.refresh();
                self.status_message = "Refreshed both panes".to_string();
            }

            if ui
                .add_sized(
                    [95.0, 38.0],
                    egui::Button::new(egui::RichText::new("📋 F5").size(13.0))
                        .fill(egui::Color32::from_rgb(55, 58, 64))
                        .rounding(6.0),
                )
                .on_hover_text("Copy selected item")
                .clicked()
            {
                self.copy_to_clipboard();
            }

            if ui
                .add_sized(
                    [95.0, 38.0],
                    egui::Button::new(egui::RichText::new("✂ F6").size(13.0))
                        .fill(egui::Color32::from_rgb(55, 58, 64))
                        .rounding(6.0),
                )
                .on_hover_text("Cut/Move selected item")
                .clicked()
            {
                self.cut_to_clipboard();
            }

            let paste_enabled = self.clipboard.is_some();
            let paste_tooltip = if paste_enabled {
                "Paste from clipboard"
            } else {
                "Clipboard is empty"
            };

            let paste_btn = ui
                .add_enabled_ui(paste_enabled, |ui| {
                    ui.add_sized(
                        [95.0, 38.0],
                        egui::Button::new(egui::RichText::new("📎 F7").size(13.0))
                            .fill(egui::Color32::from_rgb(55, 58, 64))
                            .rounding(6.0),
                    )
                })
                .inner
                .on_hover_text(paste_tooltip);

            if paste_btn.clicked() && paste_enabled {
                let _ = self.paste_from_clipboard();
            }

            if ui
                .add_sized(
                    [95.0, 38.0],
                    egui::Button::new(egui::RichText::new("🗑 F8").size(13.0))
                        .fill(egui::Color32::from_rgb(55, 58, 64))
                        .rounding(6.0),
                )
                .on_hover_text("Delete selected item")
                .clicked()
            {
                if let Some(item) = self.get_active_pane().get_selected_item() {
                    self.item_to_delete = Some(item.name.clone());
                    self.show_delete_confirm = true;
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_sized(
                        [95.0, 38.0],
                        egui::Button::new(egui::RichText::new("✗ Exit").size(13.0))
                            .fill(egui::Color32::from_rgb(184, 51, 63))
                            .rounding(6.0),
                    )
                    .clicked()
                {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }

    fn render_transfer_buttons(&mut self, ui: &mut egui::Ui) {
        // Add vertical spacing to center the buttons
        ui.add_space(ui.available_height() / 2.0 - 120.0);

        ui.vertical_centered(|ui| {
            ui.style_mut().spacing.item_spacing.y = 12.0;

            // Get selected item info
            let has_selection = self.get_active_pane().get_selected_item().is_some();
            let selected_name = self
                .get_active_pane()
                .get_selected_item()
                .map(|item| item.name.clone())
                .unwrap_or_default();
            let is_parent_dir = selected_name == "..";
            let can_transfer = has_selection && !is_parent_dir;

            // Copy Left to Right button
            let copy_left_to_right_enabled = self.active_pane == 0 && can_transfer;
            let copy_l2r_tooltip = if copy_left_to_right_enabled {
                "Copy selected item\nfrom Left to Right"
            } else {
                "Select an item in left pane"
            };

            let copy_l2r_clicked = ui
                .add_enabled_ui(copy_left_to_right_enabled, |ui| {
                    ui.add_sized(
                        [50.0, 50.0],
                        egui::Button::new(egui::RichText::new("➡\n📋").size(14.0))
                            .fill(egui::Color32::from_rgb(66, 133, 244))
                            .rounding(8.0),
                    )
                    .on_hover_text(copy_l2r_tooltip)
                })
                .inner
                .clicked();

            if copy_l2r_clicked && copy_left_to_right_enabled {
                if let Some(item) = self.left_pane.get_selected_item() {
                    let source_path = item.path.clone();
                    let dest_path = self.right_pane.current_path.join(&item.name);

                    let result = if item.is_dir {
                        self.copy_dir_recursive(&source_path, &dest_path)
                    } else {
                        fs::copy(&source_path, &dest_path)
                            .map(|_| ())
                            .map_err(|e| anyhow::Error::from(e))
                    };

                    match result {
                        Ok(_) => {
                            self.status_message = format!("Copied: {} → Right", item.name);
                            let _ = self.right_pane.refresh();
                        }
                        Err(e) => {
                            self.status_message = format!("Error copying: {}", e);
                        }
                    }
                }
            }

            // Copy Right to Left button
            let copy_right_to_left_enabled = self.active_pane == 1 && can_transfer;
            let copy_r2l_tooltip = if copy_right_to_left_enabled {
                "Copy selected item\nfrom Right to Left"
            } else {
                "Select an item in right pane"
            };

            let copy_r2l_clicked = ui
                .add_enabled_ui(copy_right_to_left_enabled, |ui| {
                    ui.add_sized(
                        [50.0, 50.0],
                        egui::Button::new(egui::RichText::new("⬅\n📋").size(14.0))
                            .fill(egui::Color32::from_rgb(66, 133, 244))
                            .rounding(8.0),
                    )
                    .on_hover_text(copy_r2l_tooltip)
                })
                .inner
                .clicked();

            if copy_r2l_clicked && copy_right_to_left_enabled {
                if let Some(item) = self.right_pane.get_selected_item() {
                    let source_path = item.path.clone();
                    let dest_path = self.left_pane.current_path.join(&item.name);

                    let result = if item.is_dir {
                        self.copy_dir_recursive(&source_path, &dest_path)
                    } else {
                        fs::copy(&source_path, &dest_path)
                            .map(|_| ())
                            .map_err(|e| anyhow::Error::from(e))
                    };

                    match result {
                        Ok(_) => {
                            self.status_message = format!("Copied: {} → Left", item.name);
                            let _ = self.left_pane.refresh();
                        }
                        Err(e) => {
                            self.status_message = format!("Error copying: {}", e);
                        }
                    }
                }
            }

            ui.add_space(20.0);

            // Move Left to Right button
            let move_left_to_right_enabled = self.active_pane == 0 && can_transfer;
            let move_l2r_tooltip = if move_left_to_right_enabled {
                "Move selected item\nfrom Left to Right"
            } else {
                "Select an item in left pane"
            };

            let move_l2r_clicked = ui
                .add_enabled_ui(move_left_to_right_enabled, |ui| {
                    ui.add_sized(
                        [50.0, 50.0],
                        egui::Button::new(egui::RichText::new("➡\n✂").size(14.0))
                            .fill(egui::Color32::from_rgb(255, 152, 0))
                            .rounding(8.0),
                    )
                    .on_hover_text(move_l2r_tooltip)
                })
                .inner
                .clicked();

            if move_l2r_clicked && move_left_to_right_enabled {
                let (source_path, item_name, dest_dir) = {
                    let item = self.left_pane.get_selected_item();
                    let dest_dir = self.right_pane.current_path.clone();
                    if let Some(item) = item {
                        (item.path.clone(), item.name.clone(), dest_dir)
                    } else {
                        return;
                    }
                };

                let dest_path = dest_dir.join(&item_name);
                match fs::rename(&source_path, &dest_path) {
                    Ok(_) => {
                        self.status_message = format!("Moved: {} → Right", item_name);
                        let _ = self.left_pane.refresh();
                        let _ = self.right_pane.refresh();
                    }
                    Err(e) => {
                        self.status_message = format!("Error moving: {}", e);
                    }
                }
            }

            // Move Right to Left button
            let move_right_to_left_enabled = self.active_pane == 1 && can_transfer;
            let move_r2l_tooltip = if move_right_to_left_enabled {
                "Move selected item\nfrom Right to Left"
            } else {
                "Select an item in right pane"
            };

            let move_r2l_clicked = ui
                .add_enabled_ui(move_right_to_left_enabled, |ui| {
                    ui.add_sized(
                        [50.0, 50.0],
                        egui::Button::new(egui::RichText::new("⬅\n✂").size(14.0))
                            .fill(egui::Color32::from_rgb(255, 152, 0))
                            .rounding(8.0),
                    )
                    .on_hover_text(move_r2l_tooltip)
                })
                .inner
                .clicked();

            if move_r2l_clicked && move_right_to_left_enabled {
                let (source_path, item_name, dest_dir) = {
                    let item = self.right_pane.get_selected_item();
                    let dest_dir = self.left_pane.current_path.clone();
                    if let Some(item) = item {
                        (item.path.clone(), item.name.clone(), dest_dir)
                    } else {
                        return;
                    }
                };

                let dest_path = dest_dir.join(&item_name);
                match fs::rename(&source_path, &dest_path) {
                    Ok(_) => {
                        self.status_message = format!("Moved: {} → Left", item_name);
                        let _ = self.left_pane.refresh();
                        let _ = self.right_pane.refresh();
                    }
                    Err(e) => {
                        self.status_message = format!("Error moving: {}", e);
                    }
                }
            }
        });
    }

    fn render_pane(&mut self, ui: &mut egui::Ui, pane_index: usize, ctx: &egui::Context, max_width: f32) {
        let is_active = self.active_pane == pane_index;
        let visuals = ctx.style().visuals.clone();
        
        // Force the UI to respect the allocated width
        ui.set_max_width(max_width);
        ui.set_width(max_width);

        // Collect all data we need before borrowing
        let (current_path, items, selected_index, sort_by, sort_order) = {
            let pane = if pane_index == 0 {
                &self.left_pane
            } else {
                &self.right_pane
            };
            (
                pane.current_path.clone(),
                pane.items.clone(),
                pane.selected_index,
                pane.sort_by,
                pane.sort_order,
            )
        };

        let frame_style = if is_active {
            egui::Frame::default()
                .fill(visuals.panel_fill)
                .stroke(egui::Stroke::new(
                    2.0,
                    visuals.widgets.active.bg_fill,
                ))
                .rounding(8.0)
                .inner_margin(0.0)
                .shadow(egui::epaint::Shadow {
                    offset: egui::vec2(0.0, 2.0),
                    blur: 8.0,
                    spread: 0.0,
                    color: egui::Color32::from_black_alpha(60),
                })
        } else {
            egui::Frame::default()
                .fill(visuals.panel_fill)
                .stroke(egui::Stroke::new(1.0, visuals.widgets.noninteractive.bg_fill))
                .rounding(8.0)
                .inner_margin(0.0)
        };

        // Use full available space but constrain width
        let available_width = ui.available_width();
        let available_height = ui.available_height();
        
        frame_style.show(ui, |ui| {
            // IMPORTANT: Set max width to prevent pane from expanding beyond allocated space
            ui.set_max_width(available_width);
            ui.set_width(available_width);
            
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0; // No vertical spacing!
                // Force this to take full height but respect width constraint
                ui.set_max_width(available_width);
                ui.set_height(available_height);

                // Header with breadcrumb navigation - Modern, compact
                let header_resp = egui::Frame::default()
                    .fill(visuals.widgets.noninteractive.bg_fill)
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(2.0, 0.0);
                        ui.set_height(20.0);
                        ui.horizontal(|ui| {
                            // Back button
                            let can_go_back = if pane_index == 0 {
                                self.left_pane.can_go_back()
                            } else {
                                self.right_pane.can_go_back()
                            };

                            let back_btn = ui
                                .add_enabled(
                                    can_go_back,
                                    egui::Button::new(egui::RichText::new("◀").size(16.0))
                                        .fill(visuals.widgets.inactive.bg_fill)
                                        .rounding(4.0)
                                        .min_size(egui::vec2(28.0, 24.0)),
                                )
                                .on_hover_text("Back (Alt+Left)");

                            if back_btn.clicked() && can_go_back {
                                let pane = if pane_index == 0 {
                                    &mut self.left_pane
                                } else {
                                    &mut self.right_pane
                                };
                                let _ = pane.navigate_back();
                            }

                            // Forward button
                            let can_go_forward = if pane_index == 0 {
                                self.left_pane.can_go_forward()
                            } else {
                                self.right_pane.can_go_forward()
                            };

                            let forward_btn = ui
                                .add_enabled(
                                    can_go_forward,
                                    egui::Button::new(egui::RichText::new("▶").size(16.0))
                                        .fill(visuals.widgets.inactive.bg_fill)
                                        .rounding(4.0)
                                        .min_size(egui::vec2(28.0, 24.0)),
                                )
                                .on_hover_text("Forward (Alt+Right)");

                            if forward_btn.clicked() && can_go_forward {
                                let pane = if pane_index == 0 {
                                    &mut self.left_pane
                                } else {
                                    &mut self.right_pane
                                };
                                let _ = pane.navigate_forward();
                            }

                            ui.add_space(6.0);
                            ui.label(egui::RichText::new("💾").size(14.0));
                            ui.add_space(4.0);

                            // Breadcrumb navigation - single line, truncate if needed
                            let components: Vec<_> = current_path.components().collect();
                            let available_width = ui.available_width();
                            let mut used_width = 0.0;

                            for (idx, component) in components.iter().enumerate() {
                                let comp_str = component.as_os_str().to_string_lossy();
                                let display_name = if comp_str == "/" {
                                    "/".to_string()
                                } else {
                                    comp_str.to_string()
                                };

                                let estimated_width = display_name.len() as f32 * 6.0 + 10.0;

                                // Stop if we're running out of space
                                if used_width + estimated_width > available_width - 30.0
                                    && idx < components.len() - 1
                                {
                                    ui.label(
                                        egui::RichText::new("...")
                                            .size(12.0)
                                            .color(egui::Color32::from_rgb(154, 160, 166)),
                                    );
                                    break;
                                }

                                if ui
                                    .add_sized(
                                        [estimated_width.min(150.0), 18.0],
                                        egui::Button::new(
                                            egui::RichText::new(&display_name)
                                                .size(15.0)
                                                .color(egui::Color32::from_rgb(138, 180, 248)),
                                        )
                                        .frame(false),
                                    )
                                    .clicked()
                                {
                                    // Navigate to clicked path segment
                                    let mut target_path = std::path::PathBuf::new();
                                    for i in 0..=idx {
                                        target_path.push(components[i]);
                                    }
                                    let pane = if pane_index == 0 {
                                        &mut self.left_pane
                                    } else {
                                        &mut self.right_pane
                                    };
                                    let _ = pane.navigate_to(target_path);
                                }

                                used_width += estimated_width;

                                if idx < components.len() - 1 {
                                    ui.label(
                                        egui::RichText::new("›")
                                            .size(11.0)
                                            .color(egui::Color32::from_rgb(154, 160, 166)),
                                    );
                                    used_width += 10.0;
                                }
                            }
                        });
                    });
                // Solid bottom divider for header
                let hr = header_resp.response.rect;
                let y = hr.bottom() - 0.5;
                ui.painter().line_segment(
                    [egui::pos2(hr.left(), y), egui::pos2(hr.right(), y)],
                    egui::Stroke {
                        width: 1.0,
                        color: visuals.widgets.noninteractive.bg_fill,
                    },
                );

                // Filter input (if filter mode is active and this is the active pane)
                if self.filter_mode && self.active_pane == pane_index {
                    egui::Frame::default()
                        .fill(visuals.widgets.inactive.bg_fill)
                        .inner_margin(egui::Margin::symmetric(10.0, 4.0))
                        .show(ui, |ui| {
                            ui.set_height(24.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("🔍")
                                        .size(13.0)
                                        .color(visuals.widgets.active.bg_fill),
                                );
                                ui.add_space(6.0);
                                let filter_response = ui.add(
                                    egui::TextEdit::singleline(if pane_index == 0 {
                                        &mut self.left_pane.filter_text
                                    } else {
                                        &mut self.right_pane.filter_text
                                    })
                                    .hint_text("Type to filter...")
                                    .desired_width(ui.available_width()),
                                );
                                filter_response.request_focus();
                            });
                        });
                }

                // Table for file list - wrapped in a constrained area
                use egui_extras::{TableBuilder, Column};

                let text_height = 28.0; // Smaller row height for compact view
                let table_width = ui.available_width();
                
                // Use allocate_ui to constrain the table width
                ui.allocate_ui(egui::Vec2::new(table_width, ui.available_height()), |ui| {
                    ui.set_max_width(table_width);
                    ui.set_clip_rect(ui.max_rect());
                    
                    // Calculate column widths to fit within table_width
                    let date_width = 140.0;
                    let size_width = 90.0;
                    let name_width = (table_width - date_width - size_width - 20.0).max(100.0);
                    
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(false) // Disable resize to prevent overflow
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .column(Column::exact(name_width).clip(true)) // Name - exact width
                        .column(Column::exact(date_width)) // Date
                        .column(Column::exact(size_width)) // Size
                        .sense(egui::Sense::click()) // Row selection
                    .header(26.0, |mut header| {
                        let sort_icon = |current_sort: crate::pane::SortBy| -> &str {
                            if sort_by == current_sort {
                                match sort_order {
                                    crate::pane::SortOrder::Ascending => " ▲",
                                    crate::pane::SortOrder::Descending => " ▼",
                                }
                            } else {
                                ""
                            }
                        };
                        
                        // Name Header
                        header.col(|ui| {
                            let text = format!("Name{}", sort_icon(crate::pane::SortBy::Name));
                            if ui.add(egui::Button::new(egui::RichText::new(text).strong()).frame(false)).clicked() {
                                let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                pane.toggle_sort(crate::pane::SortBy::Name);
                            }
                        });
                        
                        // Date Header
                        header.col(|ui| {
                             let text = format!("Modified{}", sort_icon(crate::pane::SortBy::Date));
                             if ui.add(egui::Button::new(egui::RichText::new(text).strong()).frame(false)).clicked() {
                                 let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                 pane.toggle_sort(crate::pane::SortBy::Date);
                             }
                        });

                        // Size Header
                        header.col(|ui| {
                             let text = format!("Size{}", sort_icon(crate::pane::SortBy::Size));
                             if ui.add(egui::Button::new(egui::RichText::new(text).strong()).frame(false)).clicked() {
                                 let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                 pane.toggle_sort(crate::pane::SortBy::Size);
                             }
                        });
                    })
                    .body(|mut body| {
                        // Get filter text for active pane
                        let filter_text = if pane_index == 0 {
                            self.left_pane.filter_text.to_lowercase()
                        } else {
                            self.right_pane.filter_text.to_lowercase()
                        };

                        // Filter items based on show_hidden_files setting AND filter text AND extension filters
                        let filtered_items: Vec<(usize, &crate::filesystem::FileItem)> = items
                            .iter()
                            .enumerate()
                            .filter(|(_, item)| {
                                if item.name == ".." { return true; }
                                if !filter_text.is_empty() && !item.name.to_lowercase().contains(&filter_text) { return false; }
                                if !self.should_show_file(item) { return false; }
                                if self.show_hidden_files { true } else { !item.name.starts_with('.') }
                            })
                            .collect();

                        body.rows(text_height, filtered_items.len(), |mut row| {
                            let item_idx = row.index();
                            let (i, item) = filtered_items[item_idx];

                            // Better icons based on file type
                            let (icon, icon_color) = if item.is_dir {
                                if item.name == ".." {
                                    ("⬆", egui::Color32::from_rgb(189, 147, 249))
                                } else {
                                    ("📁", egui::Color32::from_rgb(255, 184, 108))
                                }
                            } else {
                                let ext = item.path.extension().and_then(|e| e.to_str()).unwrap_or("");
                                match ext {
                                    "rs" => ("🦀", egui::Color32::from_rgb(222, 165, 132)),
                                    "py" => ("🐍", egui::Color32::from_rgb(87, 171, 224)),
                                    "js" | "ts" | "jsx" | "tsx" | "html" | "css" => ("🌐", egui::Color32::from_rgb(240, 219, 79)),
                                    "json" | "yaml" | "toml" | "xml" => ("⚙", egui::Color32::from_rgb(139, 233, 253)),
                                    "md" | "txt" => ("📝", egui::Color32::from_rgb(189, 147, 249)),
                                    "zip" | "tar" | "gz" | "rar" => ("📦", egui::Color32::from_rgb(255, 121, 198)),
                                    "png" | "jpg" | "jpeg" | "gif" => ("🖼", egui::Color32::from_rgb(80, 250, 123)),
                                    "mp3" | "wav" | "ogg" => ("🎵", egui::Color32::from_rgb(139, 233, 253)),
                                    "mp4" | "avi" | "mov" => ("🎬", egui::Color32::from_rgb(255, 121, 198)),
                                    "exe" | "sh" | "bat" => ("⚡", egui::Color32::from_rgb(80, 250, 123)),
                                    _ => ("📄", egui::Color32::from_rgb(189, 193, 198)),
                                }
                            };

                            let is_selected = i == selected_index;
                            
                            // Highlight selection
                            row.set_selected(is_selected); 

                            // Name Column
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(icon).size(16.0).color(icon_color));
                                    
                                    // Git status
                                    if let Some(ref status) = item.git_status {
                                        ui.label(egui::RichText::new(status.icon()).size(12.0).color(status.color()));
                                    }

                                    let text_color = if is_selected { egui::Color32::WHITE } else { egui::Color32::LIGHT_GRAY };
                                    ui.label(egui::RichText::new(&item.name).size(13.0).color(text_color));
                                });
                            });

                            // Date Column
                            row.col(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(egui::RichText::new(crate::filesystem::format_date(item.modified)).size(11.0).monospace());
                                });
                            });

                            // Size Column
                            row.col(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let size_str = if item.is_dir { "<DIR>".to_string() } else { crate::filesystem::format_size(item.size) };
                                    ui.label(egui::RichText::new(size_str).size(11.0).monospace());
                                });
                            });

                            // Row Interaction
                            let response = row.response();
                            if response.clicked() {
                                let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                pane.selected_index = i;
                                self.active_pane = pane_index;
                                if self.show_preview_panel { self.update_previews(); }
                            }
                            if response.double_clicked() {
                                if item.is_dir {
                                    let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                    if let Err(e) = pane.enter_directory() {
                                        self.status_message = format!("Error: {}", e);
                                    }
                                } else {
                                    let _ = self.open_file_with_default_app();
                                }
                            }
                            if response.secondary_clicked() {
                                let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                pane.selected_index = i;
                                self.active_pane = pane_index;
                                if let Some(pos) = response.interact_pointer_pos() {
                                    self.context_menu_pos = pos;
                                    self.show_context_menu = true;
                                    self.context_menu_item_index = i;
                                }
                            }
                        });
                    });
                }); // Close allocate_ui
            });
        });

        // Handle click to activate pane and ensure hover detection
        let pane_rect = ui.min_rect();
        if ui.rect_contains_pointer(pane_rect) {
            // Activate pane on click
            if ctx.input(|i| i.pointer.primary_clicked()) {
                self.active_pane = pane_index;
            }

            // Ensure this pane captures hover for scroll
            ui.interact(
                pane_rect,
                egui::Id::new(format!("pane_interact_{}", pane_index)),
                egui::Sense::hover(),
            );
        }
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            // Navigation
            if i.key_pressed(egui::Key::ArrowUp) {
                self.get_active_pane_mut().move_up();
                if self.show_preview_panel {
                    self.update_previews();
                }
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                self.get_active_pane_mut().move_down();
                if self.show_preview_panel {
                    self.update_previews();
                }
            }
            if i.key_pressed(egui::Key::Enter) {
                if let Err(e) = self.get_active_pane_mut().enter_directory() {
                    self.status_message = format!("Error: {}", e);
                }
            }

            // Tab to switch panes
            if i.key_pressed(egui::Key::Tab) {
                self.active_pane = if self.active_pane == 0 { 1 } else { 0 };
            }

            // Alt+Left - Navigate back
            if i.modifiers.alt && i.key_pressed(egui::Key::ArrowLeft) {
                if let Err(e) = self.get_active_pane_mut().navigate_back() {
                    self.status_message = format!("Error: {}", e);
                } else {
                    self.status_message = "Navigated back".to_string();
                }
            }

            // Alt+Right - Navigate forward
            if i.modifiers.alt && i.key_pressed(egui::Key::ArrowRight) {
                if let Err(e) = self.get_active_pane_mut().navigate_forward() {
                    self.status_message = format!("Error: {}", e);
                } else {
                    self.status_message = "Navigated forward".to_string();
                }
            }

            // F2 - Refresh (Total Commander style)
            if i.key_pressed(egui::Key::F2) {
                let _ = self.left_pane.refresh();
                let _ = self.right_pane.refresh();
                self.status_message = "Refreshed both panes".to_string();
            }

            // F3 - Open file with default app (Total Commander style)
            if i.key_pressed(egui::Key::F3) {
                let _ = self.open_file_with_default_app();
            }

            // F5 - Copy (Total Commander style)
            if i.key_pressed(egui::Key::F5) {
                self.copy_to_clipboard();
            }

            // F6 - Cut/Move (Total Commander style)
            if i.key_pressed(egui::Key::F6) {
                self.cut_to_clipboard();
            }

            // F7 - Paste
            if i.key_pressed(egui::Key::F7) {
                let _ = self.paste_from_clipboard();
            }

            // F8 - Delete (Total Commander style)
            if i.key_pressed(egui::Key::F8) {
                if let Some(item) = self.get_active_pane().get_selected_item() {
                    self.item_to_delete = Some(item.name.clone());
                    self.show_delete_confirm = true;
                }
            }

            // Ctrl+H - Toggle hidden files
            if i.modifiers.ctrl && i.key_pressed(egui::Key::H) {
                self.show_hidden_files = !self.show_hidden_files;
                let _ = self.left_pane.refresh();
                let _ = self.right_pane.refresh();
                self.status_message = if self.show_hidden_files {
                    "Showing hidden files".to_string()
                } else {
                    "Hiding hidden files".to_string()
                };
            }

            // Ctrl+B - Toggle sidebar
            if i.modifiers.ctrl && i.key_pressed(egui::Key::B) {
                self.show_sidebar = !self.show_sidebar;
                self.status_message = if self.show_sidebar {
                    "Sidebar shown".to_string()
                } else {
                    "Sidebar hidden".to_string()
                };
            }

            // Ctrl+F - Toggle filter mode
            if i.modifiers.ctrl && !i.modifiers.shift && i.key_pressed(egui::Key::F) {
                self.filter_mode = !self.filter_mode;
                if !self.filter_mode {
                    // Clear filter when exiting
                    self.get_active_pane_mut().filter_text.clear();
                }
                self.status_message = if self.filter_mode {
                    "Filter mode: Type to filter files".to_string()
                } else {
                    "Filter cleared".to_string()
                };
            }

            // Ctrl+Shift+F - Advanced Search
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::F) {
                self.search_criteria.search_path = self.get_active_pane().current_path.clone();
                self.show_search_dialog = true;
            }

            // ESC - Exit filter mode or close dialogs
            if i.key_pressed(egui::Key::Escape) {
                if self.filter_mode {
                    self.filter_mode = false;
                    self.get_active_pane_mut().filter_text.clear();
                    self.status_message = "Filter cleared".to_string();
                } else if self.show_new_folder_dialog {
                    self.show_new_folder_dialog = false;
                    self.new_folder_name.clear();
                } else if self.show_rename_dialog {
                    self.show_rename_dialog = false;
                    self.rename_new_name.clear();
                } else if self.show_about_dialog {
                    self.show_about_dialog = false;
                } else if self.show_properties_dialog {
                    self.show_properties_dialog = false;
                    self.properties_item = None;
                } else if self.show_delete_confirm {
                    self.show_delete_confirm = false;
                    self.item_to_delete = None;
                } else if self.show_search_dialog {
                    self.show_search_dialog = false;
                } else if self.show_compare_dialog {
                    self.show_compare_dialog = false;
                    self.comparison_result = None;
                }
            }

            // Ctrl+D - Compare files
            if i.modifiers.ctrl && i.key_pressed(egui::Key::D) {
                let _ = self.compare_selected_files();
            }

            // Ctrl+N - New folder
            if i.modifiers.ctrl && i.key_pressed(egui::Key::N) {
                self.show_new_folder_dialog = true;
            }

            // Space - Toggle preview panel and update preview
            if i.key_pressed(egui::Key::Space) && !self.filter_mode {
                self.show_preview_panel = !self.show_preview_panel;
                if self.show_preview_panel {
                    self.update_previews();
                }
            }

            // Legacy shortcuts
            if i.modifiers.ctrl && i.key_pressed(egui::Key::C) {
                self.copy_to_clipboard();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::X) {
                self.cut_to_clipboard();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::V) {
                let _ = self.paste_from_clipboard();
            }
            if i.key_pressed(egui::Key::Delete) {
                if let Some(item) = self.get_active_pane().get_selected_item() {
                    self.item_to_delete = Some(item.name.clone());
                    self.show_delete_confirm = true;
                }
            }
        });
    }

    fn update_previews(&mut self) {
        self.update_left_preview();
        self.update_right_preview();
    }

    fn update_left_preview(&mut self) {
        if let Some(item) = self.left_pane.get_selected_item() {
            self.preview_content_left = self.generate_preview_content(&item);
        } else {
            self.preview_content_left = None;
        }
    }

    fn update_right_preview(&mut self) {
        if let Some(item) = self.right_pane.get_selected_item() {
            self.preview_content_right = self.generate_preview_content(&item);
        } else {
            self.preview_content_right = None;
        }
    }

    fn generate_preview_content(&self, item: &crate::filesystem::FileItem) -> Option<PreviewContent> {
        if item.name == ".." {
            return None;
        }

        let path = &item.path;

        // Check if it's an image, text, or PDF
        if !item.is_dir {
            if let Some(ext) = path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if matches!(
                    ext_lower.as_str(),
                    "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp"
                ) {
                    return Some(PreviewContent::Image(path.clone()));
                }

                // Check if it's a PDF
                if ext_lower == "pdf" {
                    // Try to render PDF and get page count
                    let (pages, image) = match crate::pdf_renderer::render_pdf_page(path) {
                        Ok(rendered_image) => {
                            // Also get page count
                            let page_count = crate::pdf_renderer::get_page_count(path)
                                .unwrap_or(1); // Default to 1 if we can't read page count
                            (page_count, Some(rendered_image))
                        }
                        Err(e) => {
                            // Rendering failed - try to get page count for fallback
                            eprintln!("PDF preview info: {}", e);
                            let page_count = crate::pdf_renderer::get_page_count(path)
                                .unwrap_or(0); // Default to 0 if we can't read it
                            (page_count, None)
                        }
                    };

                    return Some(PreviewContent::Pdf {
                        name: item.name.clone(),
                        size: crate::filesystem::format_size(item.size),
                        modified: crate::filesystem::format_date(item.modified),
                        pages,
                        image,
                    });
                }

                // Check if it's a text file
                if matches!(
                    ext_lower.as_str(),
                    "txt"
                        | "md"
                        | "rs"
                        | "toml"
                        | "json"
                        | "xml"
                        | "html"
                        | "css"
                        | "js"
                        | "py"
                        | "c"
                        | "cpp"
                        | "h"
                        | "hpp"
                        | "sh"
                        | "yaml"
                        | "yml"
                ) {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        let lines: Vec<&str> = content.lines().take(20).collect();
                        return Some(PreviewContent::Text(lines.join("\n")));
                    }
                }
            }
        }

        // Default: show file info
        let (summary, details) = crate::filesystem::get_permissions(path)
            .unwrap_or(("N/A".to_string(), "".to_string()));

        Some(PreviewContent::FileInfo {
            name: item.name.clone(),
            size: crate::filesystem::format_size(item.size),
            modified: crate::filesystem::format_date(item.modified),
            permissions: format!("{}\n{}", summary, details),
            is_dir: item.is_dir,
        })
    }

    fn render_preview_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let visuals = ctx.style().visuals.clone();
        let panel_fill = visuals.panel_fill;
        let stroke_color = visuals.widgets.noninteractive.bg_fill;
        
        egui::Frame::default()
            .fill(panel_fill)
            .stroke(egui::Stroke::new(1.0, stroke_color))
            .rounding(8.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.set_height(188.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("👁 Dual Preview").size(14.0).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("✗").clicked() {
                            self.show_preview_panel = false;
                        }
                    });
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Fixed width layout with divider in the middle
                let available_width = ui.available_width();
                let divider_width = 2.0;
                let preview_width = (available_width - divider_width) / 2.0;

                ui.horizontal(|ui| {
                    ui.set_height(140.0);

                    // LEFT PREVIEW (fixed width)
                    ui.vertical(|ui| {
                        ui.set_width(preview_width);
                        ui.label(egui::RichText::new("Left Pane").size(11.0).strong());
                        self.render_preview_content(
                            ui,
                            ctx,
                            &self.preview_content_left.clone(),
                        );
                    });

                    // CENTER DIVIDER (fixed width)
                    ui.separator();

                    // RIGHT PREVIEW (fixed width)
                    ui.vertical(|ui| {
                        ui.set_width(preview_width);
                        ui.label(egui::RichText::new("Right Pane").size(11.0).strong());
                        self.render_preview_content(
                            ui,
                            ctx,
                            &self.preview_content_right.clone(),
                        );
                    });
                });
            });
    }

    fn render_preview_content(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        content: &Option<PreviewContent>,
    ) {
        if let Some(content) = content {
            match content {
                PreviewContent::Text(text) => {
                    egui::ScrollArea::vertical()
                        .max_height(120.0)
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(text)
                                    .family(egui::FontFamily::Monospace)
                                    .size(10.0)
                                    .color(egui::Color32::from_rgb(189, 193, 198)),
                            );
                        });
                }
                PreviewContent::Image(path) => match image::open(path) {
                    Ok(img) => {
                        let size = [img.width() as usize, img.height() as usize];
                        let rgba = img.to_rgba8();
                        let pixels = rgba.as_flat_samples();

                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            size,
                            pixels.as_slice(),
                        );

                        let texture = ctx.load_texture(
                            format!("preview_image_{}", path.display()),
                            color_image,
                            egui::TextureOptions::LINEAR,
                        );

                        let img_size = texture.size_vec2();
                        let available_width = ui.available_width();
                        let scale = (available_width / img_size.x)
                            .min(120.0 / img_size.y)
                            .min(1.0);
                        let display_size = img_size * scale;

                        ui.add(egui::Image::new(&texture).max_size(display_size));
                    }
                    Err(e) => {
                        ui.label(
                            egui::RichText::new(format!("Error: {}", e))
                                .color(egui::Color32::RED)
                                .size(10.0),
                        );
                    }
                },
                PreviewContent::Pdf {
                    name,
                    size,
                    modified,
                    pages,
                    image,
                } => {
                    ui.vertical(|ui| {
                        // File info header
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("📋").size(16.0));
                            ui.add_space(6.0);
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(name).size(11.0).strong());
                                ui.add_space(1.0);
                                ui.label(
                                    egui::RichText::new(format!("Size: {} | Pages: {}", size, pages))
                                        .size(9.0)
                                        .color(egui::Color32::from_rgb(189, 193, 198)),
                                );
                                ui.label(
                                    egui::RichText::new(format!("Modified: {}", modified))
                                        .size(9.0)
                                        .color(egui::Color32::from_rgb(189, 193, 198)),
                                );
                            });
                        });

                        ui.add_space(4.0);

                        // Display rendered image if available
                        if let Some(img) = image {
                            let img_size = [img.width() as usize, img.height() as usize];
                            let rgba = img.to_rgba8();
                            let pixels = rgba.as_flat_samples();

                            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                img_size,
                                pixels.as_slice(),
                            );

                            let texture = ctx.load_texture(
                                format!("pdf_preview_{}", name),
                                color_image,
                                egui::TextureOptions::LINEAR,
                            );

                            let img_size = texture.size_vec2();
                            let available_width = ui.available_width();
                            let available_height = ui.available_height();
                            let scale = (available_width / img_size.x)
                                .min(available_height / img_size.y)
                                .min(1.0);
                            let display_size = img_size * scale;

                            ui.add(egui::Image::new(&texture).max_size(display_size));
                        } else {
                            ui.label(
                                egui::RichText::new("PDF preview not available")
                                    .size(9.0)
                                    .color(egui::Color32::GRAY),
                            );
                            ui.label(
                                egui::RichText::new("(PDFium library may not be installed)")
                                    .size(8.0)
                                    .color(egui::Color32::DARK_GRAY),
                            );
                        }
                    });
                }
                PreviewContent::FileInfo {
                    name,
                    size,
                    modified,
                    permissions,
                    is_dir,
                } => {
                    ui.horizontal(|ui| {
                        let icon = if *is_dir { "📁" } else { "📄" };
                        ui.label(egui::RichText::new(icon).size(20.0));
                        ui.add_space(6.0);
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(name).size(12.0).strong());
                            ui.add_space(2.0);
                            ui.label(
                                egui::RichText::new(format!("Size: {}", size))
                                    .size(10.0)
                                    .color(egui::Color32::from_rgb(189, 193, 198)),
                            );
                            ui.label(
                                egui::RichText::new(format!("Modified: {}", modified))
                                    .size(10.0)
                                    .color(egui::Color32::from_rgb(189, 193, 198)),
                            );
                            ui.add_space(2.0);
                            ui.label(
                                egui::RichText::new(permissions)
                                    .family(egui::FontFamily::Monospace)
                                    .size(9.0)
                                    .color(egui::Color32::from_rgb(138, 180, 248)),
                            );
                        });
                    });
                }
            }
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(
                    egui::RichText::new("No file selected")
                        .size(11.0)
                        .color(egui::Color32::GRAY),
                );
            });
        }
    }

    fn render_sidebar(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(5.0);

                // QUICK ACCESS Section
                ui.indent("quick_access", |ui| { // Use indent for consistent left padding
                    ui.horizontal(|ui| {
                        let arrow = if self.sidebar_quick_access_expanded {
                            "▼"
                        } else {
                            "▶"
                        };
                        if ui.add_sized([24.0, 24.0], egui::Button::new(egui::RichText::new(arrow).size(14.0).color(egui::Color32::from_rgb(100, 150, 255))).frame(false)).clicked() {
                            self.sidebar_quick_access_expanded = !self.sidebar_quick_access_expanded;
                        }
                        ui.label(
                            egui::RichText::new("QUICK ACCESS")
                                .size(13.0)
                                .strong()
                                .color(egui::Color32::from_rgb(154, 160, 166)),
                        );
                    });

                    if self.sidebar_quick_access_expanded {
                        ui.add_space(5.0);
                        let quick_items = crate::bookmarks::get_quick_access_items();
                        for item in quick_items {
                            let text = format!("{} {}", item.icon, item.name);
                            let response = ui.add(
                                egui::Button::new(egui::RichText::new(text).size(14.0))
                                    .frame(false)
                                    .min_size(egui::vec2(ui.available_width(), 32.0)),
                            );

                            if response.clicked() {
                                let path = item.path.clone();
                                let _ = self.get_active_pane_mut().navigate_to(path.clone());
                                self.status_message = format!("Navigated to {}", path.display());
                            }
                        }
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // BOOKMARKS Section
                ui.indent("bookmarks", |ui| { // Use indent for consistent left padding
                    ui.horizontal(|ui| {
                        let arrow = if self.sidebar_bookmarks_expanded {
                            "▼"
                        } else {
                            "▶"
                        };
                        if ui.add_sized([24.0, 24.0], egui::Button::new(egui::RichText::new(arrow).size(14.0).color(egui::Color32::from_rgb(100, 150, 255))).frame(false)).clicked() {
                            self.sidebar_bookmarks_expanded = !self.sidebar_bookmarks_expanded;
                        }
                        ui.label(
                            egui::RichText::new("BOOKMARKS")
                                .size(13.0)
                                .strong()
                                .color(egui::Color32::from_rgb(154, 160, 166)),
                        );

                        // Add button positioned to the right
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add_sized([24.0, 24.0], egui::Button::new(egui::RichText::new("➕").size(13.0))).clicked() {
                                self.show_add_bookmark_dialog = true;
                                self.new_bookmark_name = self
                                    .get_active_pane()
                                    .current_path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Bookmark")
                                    .to_string();
                            }
                        });
                    });

                    if self.sidebar_bookmarks_expanded {
                        ui.add_space(5.0);
                        let bookmarks = self.bookmark_manager.get_bookmarks().to_vec();
                        if bookmarks.is_empty() {
                            ui.label(
                                egui::RichText::new("No bookmarks")
                                    .size(13.0)
                                    .color(egui::Color32::GRAY),
                            );
                        } else {
                            let mut remove_index: Option<usize> = None;

                            for (_idx, bookmark) in bookmarks.iter().enumerate() {
                                let text = format!("{} {}", bookmark.icon, bookmark.name);
                                let response = ui.add(
                                    egui::Button::new(egui::RichText::new(text).size(14.0))
                                        .frame(false)
                                        .min_size(egui::vec2(ui.available_width(), 32.0)),
                                );

                                if response.clicked() {
                                    let path = bookmark.path.clone();
                                    let _ = self.get_active_pane_mut().navigate_to(path.clone());
                                    self.status_message =
                                        format!("Navigated to {}", path.display());
                                }

                                // Right-click to remove
                                if response.secondary_clicked() {
                                    remove_index = Some(_idx);
                                }
                            }

                            if let Some(idx) = remove_index {
                                let _ = self.bookmark_manager.remove_bookmark(idx);
                                self.status_message = "Bookmark removed".to_string();
                            }
                        }
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // DEVICES Section
                ui.indent("devices", |ui| { // Use indent for consistent left padding
                    ui.horizontal(|ui| {
                        let arrow = if self.sidebar_devices_expanded {
                            "▼"
                        } else {
                            "▶"
                        };
                        if ui.add_sized([24.0, 24.0], egui::Button::new(egui::RichText::new(arrow).size(14.0).color(egui::Color32::from_rgb(100, 150, 255))).frame(false)).clicked() {
                            self.sidebar_devices_expanded = !self.sidebar_devices_expanded;
                        }
                        ui.label(
                            egui::RichText::new("DEVICES")
                                .size(13.0)
                                .strong()
                                .color(egui::Color32::from_rgb(154, 160, 166)),
                        );
                    });

                    if self.sidebar_devices_expanded {
                        ui.add_space(5.0);
                        let mounts = crate::filesystem::get_mount_points();
                        for mount in mounts.iter().take(5) {
                            let icon = if mount.is_removable { "🔌" } else { "💾" };
                            let label = if let Some(name) = mount.mount_point.file_name() {
                                name.to_string_lossy().to_string()
                            } else {
                                mount.mount_point.display().to_string()
                            };

                            let text = format!("{} {}", icon, label);
                            let response = ui.add(
                                egui::Button::new(egui::RichText::new(text).size(14.0))
                                    .frame(false)
                                    .min_size(egui::vec2(ui.available_width(), 32.0)),
                            );

                            if response.clicked() {
                                let path = mount.mount_point.clone();
                                let _ = self.get_active_pane_mut().navigate_to(path.clone());
                                self.status_message = format!("Navigated to {}", path.display());
                            }
                        }

                        if mounts.len() > 5 {
                            ui.add_space(5.0);
                            if ui
                                .button(egui::RichText::new("View all...").size(13.0))
                                .clicked()
                            {
                                self.show_mounts_dialog = true;
                            }
                        }
                    }
                });

                ui.add_space(10.0);
            });
    }
}
