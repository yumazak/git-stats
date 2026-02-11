mod common;

use common::tui_fixture::make_app;
use common::tui_render::render_ui;
use insta::assert_snapshot;

#[test]
fn test_ui_split_default_snapshot() {
    let app = make_app(false);
    let rendered = render_ui(&app);
    assert_snapshot!("ui_split_default", rendered);
}

#[test]
fn test_ui_single_commits_snapshot() {
    let app = make_app(true);
    let rendered = render_ui(&app);
    assert_snapshot!("ui_single_commits", rendered);
}

fn assert_single_chart_snapshot(name: &str, next_count: usize) {
    let mut app = make_app(true);
    for _ in 0..next_count {
        app.next_chart();
    }
    let rendered = render_ui(&app);
    assert_snapshot!(name, rendered);
}

#[test]
fn test_ui_single_files_changed_snapshot() {
    assert_single_chart_snapshot("ui_single_files_changed", 1);
}

#[test]
fn test_ui_single_add_del_snapshot() {
    assert_single_chart_snapshot("ui_single_add_del", 2);
}

#[test]
fn test_ui_single_weekday_snapshot() {
    assert_single_chart_snapshot("ui_single_weekday", 3);
}

#[test]
fn test_ui_single_hour_snapshot() {
    assert_single_chart_snapshot("ui_single_hour", 4);
}
