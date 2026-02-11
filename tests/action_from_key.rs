use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use kodo::tui::mvu::action::Action;

#[test]
fn maps_navigation_and_mode_keys() {
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)),
        Action::Quit
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
        Action::Quit
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)),
        Action::NextChart
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE)),
        Action::NextChart
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE)),
        Action::NextChart
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT)),
        Action::PrevChart
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE)),
        Action::PrevChart
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE)),
        Action::PrevChart
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)),
        Action::ScrollUp
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE)),
        Action::ScrollUp
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)),
        Action::ScrollDown
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE)),
        Action::ScrollDown
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE)),
        Action::ToggleMetricView
    );
}

#[test]
fn maps_force_quit_and_noop() {
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
        Action::ForceQuit
    );
    assert_eq!(
        Action::from_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
        Action::Noop
    );
}
