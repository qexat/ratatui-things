use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{block::title, Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    counter: u8,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left | KeyCode::Char('-') => self.decrement_counter(1),
            KeyCode::Right | KeyCode::Char('+') => self.increment_counter(1),
            _ => {}
        }
    }

    fn decrement_counter(&mut self, n: u8) {
        self.counter = u8::saturating_sub(self.counter, n);
    }

    fn increment_counter(&mut self, n: u8) {
        self.counter = u8::saturating_add(self.counter, n);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".light_blue().into(),
            " · ".dim().into(),
            " Increment ".into(),
            "<Right>".light_blue().into(),
            " · ".dim().into(),
            " Quit ".into(),
            "<Q>".blue().into(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buffer);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    ratatui::restore();

    app_result
}
