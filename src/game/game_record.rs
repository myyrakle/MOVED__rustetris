#[derive(Debug, Clone, Default)]
pub struct GameRecord {
    pub score: i64,         // 점수
    pub line: i32,          // 지운 줄 개수
    pub quad: i32,          // 4줄 지우기 개수
    pub tspin_single: i32,  // T스핀 싱글 횟수
    pub tspin_double: i32,  // T스핀 더블 횟수
    pub tspin_triple: i32,  // T스핀 트리플 횟수
    pub perfect_clear: i32, // 퍼펙트 클리어 횟수
    pub max_combo: i32,     // 최대 콤보
    pub back_to_back: i32,  // 최대 백투백
}
