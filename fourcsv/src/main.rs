use crossterm::event::KeyCode;
use crossterm::event;
use crossterm::event::Event;
use tui::widgets::Paragraph;
use tui::Frame;
use tui::backend::Backend;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::event::EnableMouseCapture;
use crossterm::event::DisableMouseCapture;
use crossterm::execute;
use std::thread;
use std::io;
use std::time::Duration;
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::terminal::disable_raw_mode;


fn layout<B: Backend>(f: &mut Frame<B>, user_input: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());
    
    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    
    let block = Block::default()
        .title("Type here (Press Esc to exit)")
        .borders(Borders::ALL);
    
    let main_text = Paragraph::new(user_input).block(block);
    f.render_widget(main_text, chunks[1]);
}


fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut live_input = String::new();

    loop {
        terminal.draw(|f| {
            layout(f, &live_input); 
        })?;

        if event::poll(Duration::from_millis(16))? { 
            
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break; 
                    }
                    KeyCode::Char(c) => {
                        live_input.push(c);
                    }
                    KeyCode::Backspace => {
                        live_input.pop();
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
     
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
