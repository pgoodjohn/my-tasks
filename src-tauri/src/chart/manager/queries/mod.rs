use serde::Serialize;

mod load_rolling_week_day_charts;

#[derive(Debug, Serialize)]
pub struct RollingWeekDayCharts {
    pub day: String,
    pub completed_tasks: i32,
    pub created_tasks: i32,
}
