use std::io;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal,
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::Backend,
    Frame,
};
use std::error::Error;
use std::io::prelude::*;

// Constants for block titles, margin, and percentage constraints
const BLOCK_TITLE_1: &str = "Région";
const BLOCK_TITLE_2: &str = "Servers";
const BLOCK_MARGIN: u16 = 1;
const BLOCK_PERCENTAGE: [Constraint; 2] = [
    Constraint::Percentage(20),
    Constraint::Percentage(80),
    ];

fn main() {

    // Enable raw mode and enter alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // Set up TUI backend and terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut running = true;

    while running {

        // Draw UI
        terminal.draw(draw_ui)?;

        // See if ctrl + c keys are pressed.
        if let Event::Key(key) = crossterm::event::read().unwrap() {
            if key.code == KeyCode::Char('c') && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                running = false;
            }
        }
    }

    // Restore terminal settings
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

// Draw UI using TUI
fn draw_ui<B: Backend>(frame: &mut Frame<B>) {

    // Split the frame into vertical chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(BLOCK_MARGIN)
        .constraints(BLOCK_PERCENTAGE)
        .split(frame.size());

    // Create and render the first block
    let block_1 = Block::default()
        .title(BLOCK_TITLE_1)
        .borders(Borders::ALL);
    frame.render_widget(block_1, chunks[0]);

    // Create and render the second block
    let block_2 = Block::default()
        .title(BLOCK_TITLE_2)
        .borders(Borders::ALL);
    frame.render_widget(block_2, chunks[1]);
}