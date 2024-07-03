use ratatui::{
    backend::{Backend, CrosstermBackend},
    buffer::Buffer,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Alignment, Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Style, Stylize},
    terminal::Terminal,
    widgets::{block::Title, Block, Borders, LineGauge, Padding, Paragraph, Widget},
};

pub enum State {
    DisplaySelect,
    DisplayMain,
}

pub struct App {
    pub state: State,
    pub running: bool,
}

impl App {
    pub fn new(state: State) -> Self{
        App {
            state,
            running: true,
        }
    }

    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) {
        while self.running {
            self.draw(&mut terminal);
            self.handle_event();
        }
    }

    pub fn update(&self) {
        todo!();
    }

    pub fn draw(&self, terminal: &mut Terminal<impl Backend>) {
        terminal.draw(|f| f.render_widget(self, f.size())).unwrap();
    }

    pub fn handle_event(&mut self) {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        self.running = false;
                    },
                    _ => (),
                }
            }
        }
    }   
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_select(area, buf);
    }

    
}

impl App {
    fn render_select(&self, area: Rect, buf: &mut Buffer) {
        let popup_layout = Layout::vertical([
            Constraint::Percentage((100 - 10) / 2),
            Constraint::Percentage(10),
            Constraint::Percentage((100 - 10) / 2),
        ]).split(area);

        let area = Layout::horizontal([
            Constraint::Percentage((100 - 60) / 2),
            Constraint::Percentage(60),
            Constraint::Percentage((100 - 60) / 2),
        ]).split(popup_layout[1])[1];

        Paragraph::new("")
            .block(Block::bordered().title("Select file containing the signal"))
            .gray()
            .render(area, buf);
    }
}
