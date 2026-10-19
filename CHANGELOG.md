# Changelog

## v0.3.0

### New Features
- ⚡ **Background File Operations** - Copy, move, compress dan extract berjalan di background thread
  - Progress dialog real-time (persentase, jumlah file, ukuran bytes)
  - Tombol Cancel (atau tekan ESC) untuk membatalkan operasi
  - UI tetap responsif saat operasi besar berjalan
- 🔍 **Fuzzy Filter** - Filter nama file dengan pencocokan fuzzy (subsequence ala fzf)
  - Contoh: ketik `ctm` akan cocok dengan `Cargo.toml`
- 📦 **Tar/Tar.gz Support** - Extract `.tar`, `.tar.gz`, `.tgz` (selain `.zip`)
  - Context menu "Extract Archive" otomatis mendeteksi format dari ekstensi
  - `compress_to_tar_gz()`, `compress_to_tar()`, `extract_tar_archive()` di `filesystem.rs`

### Refactor
- `src/app_gui.rs` (4548 baris) dipecah menjadi modul `src/app_gui/` (17 file)
- `update()` kini orchestrator kecil; tiap panel & dialog jadi method terpisah
- Tidak ada perubahan perilaku

### Dependencies
- Added `tar` v0.4 dan `flate2` v1.0

## Unreleased

### New Features
- ⏸️ **Pause/Resume Operasi** - Progress dialog kini punya tombol Pause/Resume di samping Cancel
  - Worker thread menunggu beruntun selama paused (cancellation tetap responsif)
- 🚦 **Operation Queue** - Operasi yang dimulai saat ada op berjalan tidak lagi gagal — masuk antrean otomatis
  - Status "⏳ Queued (N waiting)", op berikutnya mulai otomatis setelah selesai
- 🔄 **Auto-Refresh (File Watcher)** - Kedua pane pantau isi folder dengan poller 1.5s; perubahan dari luar aplikasi (membuat/hapus/rename file via shell atau app lain) me-refresh tampilan secara otomatis
  - Tanpa dependensi baru (polling mtime, tidak pakai inotify)
  - Staying out of the way: tidak refresh saat background op berjalan
  - `WatcherShared` + `spawn_dir_watcher()`
- 📏 **Directory Sizes** - Kolom size folder menampilkan total ukuran konten berjalan di background (sebelumnya `<DIR>`)
  - Hitung per folder secara lazy dengan thread (maks 6 thread berjalan), cache TTL 2 menit, indikator "…" saat menghitung
  - `filesystem::dir_size()` iterative (aman untuk tree dalam) + unit test
- ⚙️ **Permissions Editor (chmod)** - Properties dialog kini punya field edit mode oktal (e.g. 644) + tombol Apply, prefill otomatis dari mode saat ini
- 🔗 **Symlink Support** - Context menu "Symlink in other pane" membuat symlink item di folder pane lain
  - Properties menampilkan "Symlink →" target (read_link) bila item adalah link
  - `filesystem::create_symlink/is_symlink/symlink_target/set_permissions` + unit test
- ↩️ **Undo Delete** - Setelah Move to Trash muncul toast dengan tombol "Undo" (8 detik) untuk restore langsung
  - `push_toast_with_action()` + `undo_trash_moves`, multi-item restore
- ⌨️ **F1 Help Overlay** - Cheat sheet lengkap shortcut dalam dialog (menu Help → Keyboard Shortcuts)
  - Diverifikasi terhadap binding kode aktual
- 🎯 **Go to Containing Folder** - Klik hasil search kini membuka folder induk DAN menempatkan kursor pada item tersebut
  - `Pane::navigate_to_containing()` navigasi + highlight sekaligus (dengan unit test)
- 🗑️ **Delete Permanently (Shift+Delete)** - Hapus tanpa lewat Trash, dengan dialog konfirmasi merah "This cannot be undone!"
  - Context menu: "Delete Permanently" di bawah Delete biasa
  - `delete_selected_items(permanent)` menggantikan `delete_selected_file()`; recursive untuk folder (`remove_dir_all`)
  - Dialog konfirmasi adaptif: teks & tombol berubah sesuai mode (Move to Trash vs Delete Permanently)
- 🖥️ **Open Terminal Here** - Context menu, menu Commands, atau Ctrl+T membuka terminal di direktori pane aktif
  - Auto-deteksi terminal emulator: gnome-terminal, konsole, alacritty, kitty, xfce4-terminal, terminator, tilix, x-terminal-emulator
  - Cross-platform (Linux, macOS, Windows)
- 📄 **New File** - Buat file kosong dari menu Commands (Ctrl+Shift+N) atau dialog baru
  - Hint ekstensi + validasi file sudah ada

### New Features
- 🗜️ **Compress to TAR.GZ** - Context menu kini punya dua opsi compress: ZIP dan TAR.GZ
  - Compress TAR.GZ berjalan di background dengan progress per file dan tombol Cancel
  - Implementasi streaming `tar::Builder` + `flate2::GzEncoder` di `ops.rs` (`tar_dir_bg`)
  - Unit test roundtrip: compress lalu baca kembali isi arsip
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
