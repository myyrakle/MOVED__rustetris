use std::collections::VecDeque;

use gloo_timers::callback::Interval;

use crate::{
    minos::shapes::MinoShape,
    util::{random, valid_mino::valid_mino},
};

use super::{bag::BagType, game_record::GameRecord, point::Point, tetris_board::TetrisBoard};

#[derive(Debug)]
pub struct GameInfo {
    pub record: GameRecord,

    pub on_play: bool,                   //게임 진행중 여부
    pub current_position: Point,         //현재 미노 좌표
    pub current_mino: Option<MinoShape>, //현재 미노 형태
    pub freezed: bool,                   //현재 미노가 보드에 붙었는지?
    pub lose: bool,                      //현재 게임 오버 여부

    pub current_bag: VecDeque<MinoShape>, //현재 가방
    pub next_bag: VecDeque<MinoShape>,    //다음 가방

    pub tetris_board: TetrisBoard, //테트리스 보드

    pub render_interval: u64, //렌더링 시간간격(밀리초)
    pub tick_interval: u64,   //틱당 시간간격(밀리초)

    pub tick_interval_handler: Option<Interval>,
    pub render_interval_handler: Option<Interval>,

    pub bag_mode: BagType, //가방 순환 규칙 사용여부 (false면 완전 랜덤. true면 한 묶음에서 랜덤)
    pub mino_list: Vec<MinoShape>, //미노 리스트
}

impl GameInfo {
    // 가방에서 미노를 새로 가져옴.
    pub fn get_mino(&mut self) -> MinoShape {
        // 현재 가방이 비어있다면
        if self.current_bag.is_empty() {
            self.fill_current_bag();
        }

        self.current_bag.pop_front().unwrap()
    }

    // 현재 가방 채움
    fn fill_current_bag(&mut self) -> Option<()> {
        if self.next_bag.is_empty() {
            self.fill_next_bag()?;
        }

        self.current_bag = self.next_bag.clone();
        self.fill_next_bag()?;

        Some(())
    }

    // 다음 가방 채움
    fn fill_next_bag(&mut self) -> Option<()> {
        match self.bag_mode {
            BagType::SevenBag => {
                self.next_bag = random::shuffle(&self.mino_list).collect();
            }
            BagType::NoBag => {
                self.next_bag = (0..self.mino_list.len())
                    .map(|_| random::random_select(&self.mino_list))
                    .collect()
            }
        }

        Some(())
    }

    pub fn tick(&mut self) {
        let current_mino = self.current_mino;

        match current_mino {
            Some(current_mino) => {
                //current_mino;
                let current_position = self.current_position;
                let next_position = current_position.set_y(self.current_position.y + 1);

                if !valid_mino(&self.tetris_board, &current_mino, next_position) {
                    // 블럭 고정 후 현재 미노에서 제거
                    self.tetris_board
                        .write_current_mino(current_mino, current_position);
                    self.current_mino = None;
                } else {
                    self.current_position = next_position;
                }

                // TODO: 줄 지우기
            }
            None => {
                let mino = self.get_mino();
                self.current_mino = Some(mino);

                let point = Point::start_point(self.tetris_board.column_count);
                self.current_position = point;

                if !valid_mino(&self.tetris_board, &mino, point) {
                    // 패배 처리
                    self.on_play = false;
                    self.lose = true;
                } else {
                }
            }
        }
    }
}
