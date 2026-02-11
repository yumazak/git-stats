use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// User intent represented as messages for the update function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    ForceQuit,
    NextChart,
    PrevChart,
    ScrollUp,
    ScrollDown,
    ToggleMetricView,
    Tick,
    Noop,
}

impl Action {
    #[must_use]
    pub fn from_key(key: KeyEvent) -> Self {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Self::Quit,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Self::ForceQuit,
            KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => Self::NextChart,
            KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => Self::PrevChart,
            KeyCode::Up | KeyCode::Char('k') => Self::ScrollUp,
            KeyCode::Down | KeyCode::Char('j') => Self::ScrollDown,
            KeyCode::Char('m') => Self::ToggleMetricView,
            _ => Self::Noop,
        }
    }
}
