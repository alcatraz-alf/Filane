# Changelog

## Unreleased

### New Features
- ✅ **Mount Points Viewer** - View and access all mount points and storage devices
  - Auto-detection of all system mount points
  - Display device info (name, filesystem type, disk type)
  - Real-time storage usage monitoring with color-coded progress bars
  - Quick navigation to mount points
  - USB/External drive detection with special icon
  - Cross-platform support (Linux, macOS, Windows)
  - TUI: Press 'm' to toggle mount points dialog
  - GUI: Commands → Mount Points menu

### Dependencies
- Added `sysinfo` v0.30 for system disk information

## v0.1.0 - Initial Release

### Features
- ✅ Dual-pane file manager dengan 2 versi: GUI dan TUI
- ✅ GUI menggunakan egui dengan Total Commander style
- ✅ TUI menggunakan ratatui untuk terminal interface
- ✅ Smart startup: otomatis buka home directory user
- ✅ Navigation dengan keyboard dan mouse (GUI)
- ✅ Copy file antar panel
- ✅ Delete file/folder dengan konfirmasi (GUI)
- ✅ Total Commander style function keys (F2, F5, F8)
- ✅ Function button bar di GUI (seperti Total Commander)
- ✅ Status bar dengan info jumlah items
- ✅ Alternating row colors (zebra striping) di GUI
- ✅ Column headers untuk Name dan Size
- ✅ Drive/path indicator di header panel

### Keyboard Shortcuts

**TUI Version:**
- `↑`/`k` - Navigate up
- `↓`/`j` - Navigate down
- `Enter` - Open directory
- `Tab` - Switch panes
- `c` - Copy file
- `d`/`Delete` - Delete file/folder
- `r` - Refresh
- `q` - Quit

**GUI Version (Total Commander Style):**
- `F2` - Refresh both panes
- `F5` - Copy file to other pane
- `F8` - Delete file/folder
- `Tab` - Switch panes
- `Arrow Keys` - Navigate
- `Enter` - Open directory
- `Ctrl+C` - Copy (alternative)
- `Delete` - Delete (alternative)

### Technical Details
- Built with Rust 🦀
- GUI: egui + eframe
- TUI: ratatui + crossterm
- Cross-platform: Windows, Linux, macOS
- Binary sizes: ~1.3MB (TUI), ~15MB (GUI)

### Bug Fixes
- Fixed empty directory display on startup
- Now correctly opens home directory by default
- Proper error handling with fallback paths
