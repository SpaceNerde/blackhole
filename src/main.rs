// really dont want to do anything today lol....

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};
use std::io::stdout;

mod app;

#[tokio::main]
async fn main() {
    stdout().execute(EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    let mut app = app::App::new(
        app::State::DisplaySelect
    );

    app.run(terminal);

    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
