use dual_pane_fm::{app, ui};

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new()?;
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| {
            ui::draw(f, app);
            // Update scroll after drawing to get viewport height
            let viewport_height = f.size().height.saturating_sub(4) as usize; // Account for borders and status
            app.update_scroll(viewport_height);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
                    
                    match key.code {
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Char('m') => app.toggle_mounts(),
                        KeyCode::Up | KeyCode::Char('k') => {
                            if shift_pressed {
                                app.move_up_with_selection();
                            } else {
                                app.clear_selection();
                                app.move_up();
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if shift_pressed {
                                app.move_down_with_selection();
                            } else {
                                app.clear_selection();
                                app.move_down();
                            }
                        }
                        KeyCode::Enter => {
                            let _ = app.enter_directory();
                        }
                        KeyCode::Tab => app.switch_pane(),
                        KeyCode::Char('r') => {
                            let _ = app.refresh();
                        }
                        KeyCode::Char('c') => {
                            let _ = app.copy_file();
                        }
                        KeyCode::Char('v') => {
                            let _ = app.move_files();
                        }
                        KeyCode::Delete | KeyCode::Char('d') => {
                            let _ = app.delete_file();
                        }
                        KeyCode::Char(' ') => app.toggle_preview(),
                        KeyCode::Esc => app.clear_selection(),
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
