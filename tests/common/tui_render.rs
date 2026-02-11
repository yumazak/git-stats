use kodo::tui::App;
use kodo::tui::ui;
use ratatui::Terminal;
use ratatui::backend::TestBackend;

const TERM_WIDTH: u16 = 80;
const TERM_HEIGHT: u16 = 20;

pub fn make_terminal() -> Terminal<TestBackend> {
    Terminal::new(TestBackend::new(TERM_WIDTH, TERM_HEIGHT))
        .expect("test terminal should be created")
}

pub fn render_ui(app: &App) -> String {
    let mut terminal = make_terminal();
    terminal
        .draw(|frame| ui::render(frame, app))
        .expect("ui rendering should succeed");

    format!("{}", terminal.backend())
}
