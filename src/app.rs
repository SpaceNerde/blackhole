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
    widgets::{block::Title, Block, Borders, LineGauge, Padding, Paragraph, Widget, Chart, Dataset},
};
use std::path::Path;
use crate::signal;

pub enum State {
    DisplaySelect,
    DisplayMain,
}

pub struct App {
    pub state: State,
    pub running: bool,
    pub selected_signal: String,
    pub char_index: usize,
}

impl App {
    pub fn new(state: State) -> Self{
        App {
            state,
            running: true,
            selected_signal: String::new(),
            char_index: 0,
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
            if key.kind == KeyEventKind::Press { match &self.state {
                State::DisplaySelect => {
                    match key.code {
                        KeyCode::Esc => {
                            self.running = false;
                        },
                        KeyCode::Char(c) => {
                            self.selected_signal.push(c);
                        },
                        KeyCode::Backspace => {
                            self.selected_signal.pop();
                        }
                        KeyCode::Enter => {
                            self.check_path();         
                        }
                        _ => (),
                    }
                },
                State::DisplayMain => {

                }
                }
            }
        }
    }   
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            State::DisplaySelect => self.render_select(area, buf),
            State::DisplayMain => self.render_main(area, buf),
        }
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

        Paragraph::new(self.selected_signal.clone())
            .block(Block::bordered().title("Select file containing the signal"))
            .gray()
            .render(area, buf);
    }

    fn render_main(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Percentage(100), 
        ]);

        self.render_main_chart(area, buf);
    }

    fn render_main_chart(&self, area: Rect, buf: &mut Buffer) {
        let data_points = signal::create_data_points(self.selected_signal.clone());
        let chart = Chart::new(vec![
            Dataset::default().data(&data_points[0]),
        ]);
        
        chart.render(area, buf)
    }
}

impl App {
    fn check_path(&mut self) {
        match Path::new(&self.selected_signal).exists() {
            true => {
                self.state = State::DisplayMain;
            },
            false => {
                self.selected_signal = "path does not exsist!".to_string();
            },
        }
    }
}
