use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use zxcvbn::zxcvbn;

mod password_generator;
mod storage_manager;

use password_generator::PasswordGenerator;
use storage_manager::StorageManager;

fn main() -> io::Result<()> {
    // Setup TUI
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app components
    let password_generator = PasswordGenerator::new();
    let mut storage_manager = StorageManager::new();
    const KEYBINDING_MESSAGE: &str = "Press 'g' to generate a password, 'q' to quit.";
    
    // UI state
    let mut current_password = String::new();
    let mut password_strength_info = String::new();

    // Main app loop
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Length(4)
                ].as_ref())
                .split(size);

            // Main password display
            let password_paragraph = Paragraph::new(if !current_password.is_empty() {
                current_password.clone()
            } else {
                "Press 'g' to generate a password".to_string()
            })
            .style(Style::default().fg(Color::White))
            .block(Block::default().title("rustle-tui Memorable Password Generator").borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

            // Bottom panel with strength info and keybindings
            let mut bottom_text = String::new();
            if !current_password.is_empty() {
                bottom_text.push_str(&password_strength_info);
                bottom_text.push_str("\n\n");
            }
            bottom_text.push_str(KEYBINDING_MESSAGE);

            let bottom_paragraph = Paragraph::new(bottom_text)
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            f.render_widget(password_paragraph, chunks[0]);
            f.render_widget(bottom_paragraph, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('g') => {
                        loop {
                            let new_password = password_generator.generate();
                            if !storage_manager.is_duplicate(&new_password) {
                                storage_manager.store_password(&new_password);
                                current_password = new_password.clone();
                                let zxcvbn_result = zxcvbn(&new_password, &[]).unwrap();
                                password_strength_info = format!(
                                    "zxcvbn caculation | Strength: {}/4 | Bruteforce time: {}",
                                    zxcvbn_result.score(),
                                    format!("{}", zxcvbn_result.crack_times().online_no_throttling_10_per_second())
                                );
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    Ok(())
}
