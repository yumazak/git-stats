/// Chart type to display in single mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChartType {
    #[default]
    Commits,
    FilesChanged,
    AddDel,
    Weekday,
    Hour,
}

impl ChartType {
    /// Get the next chart type in the cycle.
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Commits => Self::FilesChanged,
            Self::FilesChanged => Self::AddDel,
            Self::AddDel => Self::Weekday,
            Self::Weekday => Self::Hour,
            Self::Hour => Self::Commits,
        }
    }

    /// Get the previous chart type in the cycle.
    #[must_use]
    pub fn prev(self) -> Self {
        match self {
            Self::Commits => Self::Hour,
            Self::FilesChanged => Self::Commits,
            Self::AddDel => Self::FilesChanged,
            Self::Weekday => Self::AddDel,
            Self::Hour => Self::Weekday,
        }
    }

    /// Get display name.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Commits => "Commits",
            Self::FilesChanged => "Files Changed",
            Self::AddDel => "Add/Del",
            Self::Weekday => "Weekday",
            Self::Hour => "Hour",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChartType;

    #[test]
    fn chart_type_cycle() {
        let chart = ChartType::Commits;
        assert_eq!(chart.next(), ChartType::FilesChanged);
        assert_eq!(chart.next().next(), ChartType::AddDel);
        assert_eq!(chart.next().next().next(), ChartType::Weekday);
        assert_eq!(chart.next().next().next().next(), ChartType::Hour);
        assert_eq!(chart.next().next().next().next().next(), ChartType::Commits);
    }

    #[test]
    fn chart_type_prev_cycle() {
        let chart = ChartType::Commits;
        assert_eq!(chart.prev(), ChartType::Hour);
        assert_eq!(chart.prev().prev(), ChartType::Weekday);
        assert_eq!(chart.prev().prev().prev(), ChartType::AddDel);
        assert_eq!(chart.prev().prev().prev().prev(), ChartType::FilesChanged);
        assert_eq!(chart.prev().prev().prev().prev().prev(), ChartType::Commits);
    }

    #[test]
    fn chart_type_name() {
        assert_eq!(ChartType::Commits.name(), "Commits");
        assert_eq!(ChartType::FilesChanged.name(), "Files Changed");
        assert_eq!(ChartType::AddDel.name(), "Add/Del");
        assert_eq!(ChartType::Weekday.name(), "Weekday");
        assert_eq!(ChartType::Hour.name(), "Hour");
    }

    #[test]
    fn chart_type_default() {
        assert_eq!(ChartType::default(), ChartType::Commits);
    }
}
