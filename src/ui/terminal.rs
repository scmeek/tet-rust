use std::io::{self, Stderr};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
};
use ratatui::{
    buffer::Buffer,
    crossterm::terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    layout::Rect,
    prelude::CrosstermBackend,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    Frame, Terminal,
};

use crate::core::State;
use crate::ui::UI;

pub struct TerminalUI {
    terminal: Terminal<CrosstermBackend<Stderr>>,
}

impl UI for TerminalUI {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
        execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stderr);

        Ok(Self {
            terminal: Terminal::new(backend)?,
        })
    }

    fn render(&self, state: &State) {
        todo!()
    }

    fn finalize(mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

// impl App {
//     /// runs the application's main loop until the user quits
//     pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
//         while !self.exit {
//             terminal.draw(|frame| self.draw(frame))?;
//             self.handle_events()?;
//         }
//         Ok(())
//     }
//
//     fn draw(&self, frame: &mut Frame) {
//         frame.render_widget(self, frame.area());
//     }
//
//     /// updates the application's state based on user input
//     fn handle_events(&mut self) -> io::Result<()> {
//         // event::read is blocking, may want to use event::poll instead
//         match event::read()? {
//             // it's important to check that the event is a key press event as
//             // crossterm also emits key release and repeat events on Windows.
//             Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
//                 self.handle_key_event(key_event)
//             }
//             _ => {}
//         };
//         Ok(())
//     }
//
//     fn handle_key_event(&mut self, key_event: KeyEvent) {
//         match key_event.code {
//             KeyCode::Char('q') => self.exit(),
//             KeyCode::Left => self.decrement_counter(),
//             KeyCode::Right => self.increment_counter(),
//             _ => {}
//         }
//     }
//
//     fn exit(&mut self) {
//         self.exit = true;
//     }
//
//     fn increment_counter(&mut self) {
//         self.counter += 1;
//     }
//
//     fn decrement_counter(&mut self) {
//         self.counter -= 1;
//     }
// }
//
// impl Widget for &App {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         let title = Line::from(" Counter App Tutorial ".bold());
//         let instructions = Line::from(vec![
//             " Decrement ".into(),
//             "<Left>".blue().bold(),
//             " Increment ".into(),
//             "<Right>".blue().bold(),
//             " Quit ".into(),
//             "<Q> ".blue().bold(),
//         ]);
//         let block = Block::bordered()
//             .title(title.centered())
//             .title_bottom(instructions.centered())
//             .border_set(border::THICK);
//
//         let counter_text = Text::from(vec![Line::from(vec![
//             "Value: ".into(),
//             self.counter.to_string().yellow(),
//         ])]);
//
//         Paragraph::new(counter_text)
//             .centered()
//             .block(block)
//             .render(area, buf);
//     }
// }
