use std::collections::VecDeque;

use crate::minos::shapes::MinoType;

use super::{point::Point, tetris_board::TetrisBoard, tetris_cell::TetrisCell};

pub struct GameInfo {
    pub game_score: u64, //게임 점수

    pub on_play: bool,                   //게임 진행중 여부
    pub current_position: Option<Point>, //현재 미노 좌표
    pub current_mino: Option<MinoType>,  //현재 미노 형태
    pub freezed: bool,                   //현재 미노가 보드에 붙었는지?
    pub lose: bool,                      //현재 게임 오버 여부

    pub current_bag: VecDeque<MinoType>, //현재 가방
    pub next_bag: VecDeque<MinoType>,    //다음 가방

    pub tetris_board: TetrisBoard, //테트리스 보드

    pub render_interval: u64, //렌더링 시간간격(밀리초)
    pub tick_interval: u64,   //틱당 시간간격(밀리초)
}
