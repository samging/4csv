use std::io::{self, Stdout};
use std::time::{Duration, Instant};
use std::thread::sleep;

use crossterm::cursor;
use crossterm::event::KeyCode;
use crossterm::event;
use crossterm::event::Event;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::event::EnableMouseCapture;
use crossterm::event::DisableMouseCapture;

use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::Frame;
use tui::backend::Backend;

use crossterm::execute;
use std::thread;
use crossterm::terminal::disable_raw_mode;
use tui::style::Style;
use tui::style::Color;
use tui::widgets::BorderType;
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};


fn layout<B: Backend>(f: &mut Frame<B>, lines: &[String], cursor_x: usize, cursor_y: usize, set_search: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(60),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());
    
    let text_content = lines.join("\n");
    
    
    let block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_style(Style::default().fg(Color::Black))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::White))
        .borders(Borders::ALL);
    
    f.render_widget(block, chunks[0]);
     
     
    let width_chunk = Layout::default().direction(Direction::Horizontal).margin(0)
        .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
        .split(chunks[0])[0];
    
    let button_chunk = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(width_chunk)[0];
    
        
    let width_chunk_search = Layout::default().direction(Direction::Horizontal).margin(0)
        .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
        .split(chunks[0])[1];
     
    let search_chunk = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(width_chunk_search)[0];
    
    let mut block_button = Block::default()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);
   
   let search_field = Block::default().title("search")
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL); 
   
   if set_search {
    block_button = Block::default()
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black).fg(Color::Blue)) 
        .border_style(Style::default().fg(Color::LightBlue))
        .borders(Borders::ALL);
    } 
    
    f.render_widget(search_field, search_chunk);
    f.render_widget(block_button, button_chunk);
    
    let block = Block::default().title("Type here (Press Esc to exit)").borders(Borders::ALL);
    
    let main_text = Paragraph::new(text_content).block(block);
    f.render_widget(main_text, chunks[1]);
}

pub struct Timeblock { 
    from: Option<Instant>,
    to: Option<Instant>
}

impl Timeblock {
    fn set(&self) -> Self { 
        Timeblock { 
                from: Some(Instant::now()), 
                to: None
            } 
    }
    fn set_to(&self, mark: Instant) -> Self {
            Timeblock { 
                    from: self.from,
                    to: Some(mark) 
            }
    }
    fn compare_time_event(&self, timer: Duration) -> bool {
        if let (Some(start), Some(end)) = (self.from, self.to) {
            end.duration_since(start) < timer
        } else {
            false 
        }
    }
    
    fn empty(&self) -> Self {
         Timeblock { from:  None, to: None }
    }
}
        
fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut lines = vec![String::new()];
    let mut cursor_x: usize = 0;
    let mut cursor_y: usize = 0;
    let mut current_block = Timeblock { from: None, to: None };
    let mut set_search: bool = false;
    
    loop {
        terminal.draw(|f| {
            layout(f, &lines, cursor_x, cursor_y, set_search); 
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
                        
                        if current_block.from.is_some() {
                            current_block = current_block.set_to(Instant::now());
                            
                            if current_block.compare_time_event(Duration::from_millis(300)) {
                                set_search = true;
                                lines[cursor_y].push('T');
                            }
                            
                            current_block = current_block.empty();  
                        } else {
                            current_block = current_block.set()
                        }
                         
                        
                            
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
