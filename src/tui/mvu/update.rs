use crate::tui::mvu::action::Action;
use crate::tui::mvu::model::Model;

/// Pure transition function for UI state.
#[must_use]
pub fn update(mut model: Model, action: Action) -> Model {
    match action {
        Action::Quit | Action::ForceQuit => {
            model.should_quit = true;
        }
        Action::NextChart => {
            if model.single_metric {
                model.chart_type = model.chart_type.next();
            }
        }
        Action::PrevChart => {
            if model.single_metric {
                model.chart_type = model.chart_type.prev();
            }
        }
        Action::ScrollUp => {
            if model.can_scroll() && model.data_len > 0 {
                let max_offset = model.data_len.saturating_sub(1);
                model.scroll_offset = (model.scroll_offset + 1).min(max_offset);
            }
        }
        Action::ScrollDown => {
            if model.can_scroll() {
                model.scroll_offset = model.scroll_offset.saturating_sub(1);
            }
        }
        Action::ToggleMetricView => {
            model.single_metric = !model.single_metric;
            model.scroll_offset = 0;
        }
        Action::Tick | Action::Noop => {}
    }

    model
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::chart_type::ChartType;

    fn model() -> Model {
        Model {
            chart_type: ChartType::Commits,
            should_quit: false,
            single_metric: false,
            scroll_offset: 0,
            data_len: 5,
        }
    }

    #[test]
    fn update_sets_quit_flag() {
        let next = update(model(), Action::Quit);
        assert!(next.should_quit);
    }

    #[test]
    fn update_force_quit_sets_quit_flag() {
        let next = update(model(), Action::ForceQuit);
        assert!(next.should_quit);
    }

    #[test]
    fn update_next_chart_only_in_single_mode() {
        let split = update(model(), Action::NextChart);
        assert_eq!(split.chart_type, ChartType::Commits);

        let mut single = model();
        single.single_metric = true;
        let next = update(single, Action::NextChart);
        assert_eq!(next.chart_type, ChartType::FilesChanged);
    }

    #[test]
    fn update_prev_chart_only_in_single_mode() {
        let split = update(model(), Action::PrevChart);
        assert_eq!(split.chart_type, ChartType::Commits);

        let mut single = model();
        single.single_metric = true;
        let next = update(single, Action::PrevChart);
        assert_eq!(next.chart_type, ChartType::Hour);
    }

    #[test]
    fn update_scroll_up_respects_upper_bound() {
        let mut m = model();
        m.scroll_offset = 4;

        let next = update(m, Action::ScrollUp);
        assert_eq!(next.scroll_offset, 4);
    }

    #[test]
    fn update_scroll_down_saturates_at_zero() {
        let next = update(model(), Action::ScrollDown);
        assert_eq!(next.scroll_offset, 0);
    }

    #[test]
    fn update_scroll_ignored_when_single_mode_non_add_del() {
        let mut m = model();
        m.single_metric = true;
        m.chart_type = ChartType::Commits;
        let next = update(m, Action::ScrollUp);
        assert_eq!(next.scroll_offset, 0);
    }

    #[test]
    fn update_toggle_metric_view_resets_scroll_offset() {
        let mut m = model();
        m.scroll_offset = 3;

        let next = update(m, Action::ToggleMetricView);
        assert!(next.single_metric);
        assert_eq!(next.scroll_offset, 0);
    }
}
