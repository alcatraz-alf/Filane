use crate::pane::Pane;
use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

mod bars;
mod context_menu;
mod dialogs_file;
mod dialogs_info;
mod dialogs_ops;
mod dialogs_search;
mod dialogs_system;
mod image_viewer;
mod keys;
mod main_area;
mod menu;
mod pane_view;
mod preview;
mod sidebar;
mod sidebar_panel;
mod theme;

#[derive(Clone, Debug)]
pub(crate) enum ClipboardOperation {
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
    pub(crate) clipboard: Option<(std::path::PathBuf, ClipboardOperation)>,
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
    pub op_runner: Option<crate::ops::OpRunner>,
    pub show_progress_dialog: bool,
    pub op_result_message: Option<String>,
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
            op_runner: None,
            show_progress_dialog: false,
            op_result_message: None,
        }
    }

    pub fn start_operation(&mut self, op: crate::ops::Operation) {
        if self.op_runner.is_some() {
            self.status_message = "Another operation is already running".to_string();
            return;
        }
        match crate::ops::OpRunner::start(op) {
            Ok(runner) => {
                self.op_runner = Some(runner);
                self.show_progress_dialog = true;
                self.op_result_message = None;
            }
            Err(e) => {
                self.status_message = format!("❌ Failed to start operation: {}", e);
            }
        }
    }

    pub fn poll_operation(&mut self, ctx: &egui::Context) {
        let mut finished = false;
        if let Some(runner) = &self.op_runner {
            ctx.request_repaint();
            if runner.is_finished() {
                finished = true;
            }
        }
        if finished {
            if let Some(mut runner) = self.op_runner.take() {
                runner.join();
                let snap = runner.snapshot();
                if let Some(err) = &snap.error {
                    self.status_message = format!("❌ {} failed: {}", snap.kind.label(), err);
                    self.op_result_message = Some(format!("Operation failed: {}", err));
                } else if snap.cancelled {
                    self.status_message = "⚠️ Operation cancelled".to_string();
                    self.op_result_message = Some("Operation cancelled".to_string());
                } else {
                    self.status_message = format!(
                        "✅ {} complete: {} ({} files)",
                        snap.kind.label(),
                        snap.title,
                        snap.files_total
                    );
                    self.op_result_message = Some(format!(
                        "✅ Completed successfully\n\n{} → {}",
                        snap.kind.label(),
                        snap.title
                    ));
                }
            }
            self.show_progress_dialog = true; // keep dialog to show result
            let _ = self.left_pane.refresh();
            let _ = self.right_pane.refresh();
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

            if dest_path.exists() {
                self.status_message =
                    format!("❌ Destination already exists: {}", file_name);
                anyhow::bail!("Destination already exists");
            }

            let op = match operation {
                ClipboardOperation::Copy => crate::ops::Operation::Copy {
                    sources: vec![source_path.clone()],
                    dest: target_path,
                },
                ClipboardOperation::Cut => crate::ops::Operation::Move {
                    sources: vec![source_path.clone()],
                    dest: target_path,
                },
            };

            // Cut clipboard is consumed; copy clipboard persists for repeated paste
            if matches!(operation, ClipboardOperation::Cut) {
                self.clipboard = None;
            }
            self.start_operation(op);
        } else {
            self.status_message = "Clipboard is empty".to_string();
        }
        Ok(())
    }

    /// Transfer selected items from one pane to the other (background op).
    /// `op` is Copy or Move; `from_pane` is 0 (left) or 1 (right).
    pub fn transfer_selected(&mut self, from_pane: usize, op: crate::ops::Operation) {
        let (sources, dest) = {
            let (src_pane, dst_pane) = if from_pane == 0 {
                (&self.left_pane, &self.right_pane)
            } else {
                (&self.right_pane, &self.left_pane)
            };
            let sources: Vec<PathBuf> = src_pane
                .get_selected_items()
                .iter()
                .filter(|item| item.name != "..")
                .map(|item| item.path.clone())
                .collect();
            (sources, dst_pane.current_path.clone())
        };

        if sources.is_empty() {
            self.status_message = "No items selected".to_string();
            return;
        }

        let op = match op {
            crate::ops::Operation::Copy { .. } => crate::ops::Operation::Copy { sources, dest },
            _ => crate::ops::Operation::Move { sources, dest },
        };
        self.start_operation(op);
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
        let items_to_delete: Vec<_> = self.get_active_pane()
            .get_selected_items()
            .iter()
            .filter(|item| item.name != "..")
            .map(|item| (item.path.clone(), item.is_dir, item.name.clone()))
            .collect();

        let mut deleted_count = 0;
        for (path, is_dir, _name) in items_to_delete {
            if is_dir {
                fs::remove_dir_all(&path)?;
            } else {
                fs::remove_file(&path)?;
            }
            deleted_count += 1;
        }

        self.status_message = if deleted_count == 1 {
            format!("Deleted 1 item")
        } else {
            format!("Deleted {} items", deleted_count)
        };
        
        self.get_active_pane_mut().clear_selection();
        self.get_active_pane_mut().refresh()?;
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

                if zip_path.exists() {
                    self.status_message = format!("❌ Archive already exists: {}", zip_name);
                    anyhow::bail!("Archive already exists");
                }

                self.start_operation(crate::ops::Operation::CompressZip {
                    source: item.path.clone(),
                    dest_zip: zip_path,
                });
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
                    self.start_operation(crate::ops::Operation::ExtractZip {
                        archive: item.path.clone(),
                        dest_dir: extract_dir.to_path_buf(),
                    });
                } else if crate::filesystem::is_tar_gz(&item.path)
                    || crate::filesystem::is_tar(&item.path)
                {
                    self.start_operation(crate::ops::Operation::ExtractTar {
                        archive: item.path.clone(),
                        dest_dir: extract_dir.to_path_buf(),
                    });
                } else {
                    self.status_message =
                        "Only .zip, .tar.gz, .tgz and .tar files are supported".to_string();
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
        // Poll background operation
        self.poll_operation(ctx);

        // Apply modern visual style
        ctx.set_visuals(self.get_modern_visuals());
        ctx.set_zoom_factor(1.1);
        self.render_menu_bar(ctx);
        self.render_filter_bar(ctx);
        self.render_function_bar_panel(ctx);
        self.render_status_bar(ctx);

        self.render_sidebar_widgets(ctx);
        self.render_main_area(ctx);
        self.handle_keyboard(ctx);

        self.render_image_viewer(ctx);
        self.render_context_menu(ctx);

        self.render_properties_dialog(ctx);
        self.render_new_folder_dialog(ctx);
        self.render_rename_dialog(ctx);
        self.render_about_dialog(ctx);
        self.render_theme_selector(ctx);
        self.render_search_dialog(ctx);
        self.render_compare_dialog(ctx);
        self.render_mounts_dialog(ctx);
        self.render_bookmark_dialog(ctx);
        self.render_delete_confirm(ctx);
        self.render_progress_dialog(ctx);
    }
}

