use crossterm::cursor;
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


fn layout<B: Backend>(f: &mut Frame<B>, lines: &[String], cursor_x: usize, cursor_y: usize) {
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
    let text_content = lines.join("\n");
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    
    let block = Block::default().title("Type here (Press Esc to exit)").borders(Borders::ALL);
    
    let main_text = Paragraph::new(text_content).block(block);
    f.render_widget(main_text, chunks[1]);
}


fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    //let mut live_input = String::new();
    let mut lines = vec![String::new()];
    let mut cursor_x: usize = 0;
    let mut cursor_y: usize = 0;
    
    loop {
        terminal.draw(|f| {
            layout(f, &lines, cursor_x, cursor_y); 
        })?;

        if event::poll(Duration::from_millis(16))? { 
            
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break; 
                    }
                    KeyCode::Char(c) => {
                        lines[cursor_y].push(c);
                        cursor_x += 1;
                    }
                    KeyCode::Backspace => {
                        if cursor_x > 0 {
                            lines[cursor_y].pop();
                            cursor_x -= 1;
                        } else if cursor_y > 0 {
                            cursor_y -= 1;
                            cursor_x = lines[cursor_y].len();
                            lines.pop();
                        }
                    }
                    KeyCode::Enter => {
                        lines.push(String::new());
                        cursor_y += 1;
                        cursor_x = 0;
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
