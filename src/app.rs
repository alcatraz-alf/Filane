use crate::pane::Pane;
use anyhow::Result;
use std::env;
use std::fs;

pub struct App {
    pub left_pane: Pane,
    pub right_pane: Pane,
    pub active_pane: usize,
    pub should_quit: bool,
    pub show_mounts: bool,
    pub show_preview: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        // Try to get home directory first, fallback to current dir
        let start_dir = dirs::home_dir()
            .or_else(|| env::current_dir().ok())
            .unwrap_or_else(|| std::path::PathBuf::from("."));

        let left_pane = Pane::new(start_dir.clone())?;
        let right_pane = Pane::new(start_dir)?;

        Ok(App {
            left_pane,
            right_pane,
            active_pane: 0,
            should_quit: false,
            show_mounts: false,
            show_preview: false,
        })
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

    pub fn switch_pane(&mut self) {
        self.active_pane = if self.active_pane == 0 { 1 } else { 0 };
    }

    pub fn toggle_mounts(&mut self) {
        self.show_mounts = !self.show_mounts;
    }

    pub fn move_up(&mut self) {
        self.get_active_pane_mut().move_up();
    }

    pub fn move_down(&mut self) {
        self.get_active_pane_mut().move_down();
    }

    pub fn enter_directory(&mut self) -> Result<()> {
        self.get_active_pane_mut().enter_directory()
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.left_pane.refresh()?;
        self.right_pane.refresh()?;
        Ok(())
    }

    pub fn copy_file(&mut self) -> Result<()> {
        let source_pane = self.get_active_pane();
        let target_pane = if self.active_pane == 0 {
            &self.right_pane
        } else {
            &self.left_pane
        };

        if let Some(item) = source_pane.get_selected_item() {
            if !item.is_dir && item.name != ".." {
                let dest_path = target_pane.current_path.join(&item.name);
                fs::copy(&item.path, &dest_path)?;
                // Refresh both panes
                self.left_pane.refresh()?;
                self.right_pane.refresh()?;
            }
        }
        Ok(())
    }

    pub fn delete_file(&mut self) -> Result<()> {
        let active_pane = self.get_active_pane_mut();

        if let Some(item) = active_pane.get_selected_item().cloned() {
            if item.name != ".." {
                if item.is_dir {
                    fs::remove_dir_all(&item.path)?;
                } else {
                    fs::remove_file(&item.path)?;
                }
                active_pane.refresh()?;
            }
        }
        Ok(())
    }

    pub fn update_scroll(&mut self, viewport_height: usize) {
        self.left_pane.update_scroll(viewport_height);
        self.right_pane.update_scroll(viewport_height);
    }

    pub fn toggle_preview(&mut self) {
        self.show_preview = !self.show_preview;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
