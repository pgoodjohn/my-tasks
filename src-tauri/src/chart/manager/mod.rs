use sqlx::SqlitePool;

pub mod queries;

pub struct ChartManager<'a> {
    db_pool: &'a SqlitePool,
}

impl<'a> ChartManager<'a> {
    pub fn new(db_pool: &'a SqlitePool) -> Self {
        ChartManager { db_pool }
    }
}
