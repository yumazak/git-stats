use crate::tui::app::ChartType;

/// UI state for MVU update function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Model {
    pub chart_type: ChartType,
    pub should_quit: bool,
    pub single_metric: bool,
    pub scroll_offset: usize,
    pub data_len: usize,
}

impl Model {
    #[must_use]
    pub fn can_scroll(self) -> bool {
        if self.single_metric {
            matches!(self.chart_type, ChartType::AddDel)
        } else {
            true
        }
    }
}
