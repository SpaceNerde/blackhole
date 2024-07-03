use ratatui::Terminal;
use ratatui::prelude::Backend;
use ratatui::prelude::Rect;
use ratatui::prelude::Buffer;
use ratatui::widgets::Widget;

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
        }
    }

    pub fn update(&self) {
        todo!();
    }

    pub fn draw(&self, terminal: &mut Terminal<impl Backend>) {
        terminal.draw(|f| f.render_widget(self, f.size())).unwrap();
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
    }
}
