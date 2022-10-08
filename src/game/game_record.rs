#[derive(Debug, Clone, Default)]
pub struct GameRecord {
    pub score: i64,
    pub quad: i32,
    pub tspin_single: i32,
    pub tspin_double: i32,
    pub tspin_triple: i32,
    pub perfect_clear: i32,
}
