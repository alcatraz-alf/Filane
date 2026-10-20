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

## v0.3.1

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

## v0.3.3

### New Features
- 🧩 **Menu bar restrukturisasi**: **File** (operasi file: new/open/clipboard/rename/delete/exit),
  **View** (toggle checkbox + Themes + Refresh), **⭐ Bookmarks**, **🔧 Tools** (Search/Compare/Mounts),
  **❓ Help** - menggantikan Files/Commands yang campur aduk (14 item)

### UX Polish
- 🖱 **Context Menu Dirapikan** - Grup logis + hint shortcut rata kanan + entry Paste + baris Cancel dihapus
- 🔕 **Toast hanya untuk error/warning** - Pesan status rutin cukup di status bar
- 🎨 **Theme-aware Palette untuk semua elemen custom** - `Palette` struct per theme (bg/hover/text/accent/
  semantic); dialogs, toast, help overlay, context menu, sidebar kini ikut Dracula/Nord/Monokai/dll
  (104 hardcoded call site diganti)
- 🎨 **Fix: window/panel fill Dark & Light** - Theme Selector, menu bar, side panel tidak lagi jatuh ke
  default egui #1b1b1b
- 👁 **Fix: Dual Preview pakai palette** - Header, kolom Left/Right, teks info, placeholder error
- 🗓 **Fix: header kolom tabel Name/Modified/Size** - ikut theme, tidak lagi hitam konstan

## v0.3.2

### New Features
- 🔠 **UI Scale / Font Size Setting** - Pengaturan ukuran font antarmuka tersendiri
  - Slider 70%-200% (+/− presets) di Theme Selector, persist di AppPrefs
  - Shortcut Ctrl+= / Ctrl-= dengan feedback status, tercatat di F1 cheat sheet
- 📌 **Paste di Context Menu** - Muncul otomatis saat clipboard berisi item

### UX Polish
- 🖱 **Context Menu Dirapikan** - Grup logis (Open/Terminal → Clipboard → Archive → Delete → Properties),
  hint shortcut rata kanan abu-abu, hover merata, baris Cancel dihapus (tutup via klik luar/ESC)
- 🔕 **Toast hanya untuk error/warning** - Pesan status rutin (ganti tema, toggle, navigasi) cukup di status bar

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
