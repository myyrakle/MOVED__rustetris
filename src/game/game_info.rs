use std::collections::VecDeque;

use gloo_timers::callback::Interval;

use crate::game::{
    valid_mino, BagType, ClearInfo, GameRecord, MinoShape, Point, SpinType, TetrisBoard, TetrisCell,
};

use crate::util::{random, rotate_left, rotate_right, KICK_INDEX_3BY3, KICK_INDEX_I};

use super::Mino;

#[derive(Debug)]
pub struct GameInfo {
    pub record: GameRecord,

    pub on_play: bool,                   //게임 진행중 여부
    pub current_position: Point,         //현재 미노 좌표
    pub current_mino: Option<MinoShape>, //현재 미노 형태

    pub freezed: bool, //현재 미노가 보드에 붙었는지?
    pub lose: bool,    //현재 게임 오버 여부

    pub next_count: i32,          // 넥스트 개수
    pub bag: VecDeque<MinoShape>, // 현재 가방

    pub tetris_board: TetrisBoard, //테트리스 보드

    pub render_interval: u64, //렌더링 시간간격(밀리초)
    pub tick_interval: u64,   //틱당 시간간격(밀리초)

    pub tick_interval_handler: Option<Interval>,
    pub render_interval_handler: Option<Interval>,

    pub bag_mode: BagType, //가방 순환 규칙 사용여부 (false면 완전 랜덤. true면 한 묶음에서 랜덤)
    pub mino_list: Vec<MinoShape>, //미노 리스트

    pub hold: Option<MinoShape>, // 홀드한 미노
    pub hold_used: bool,         // 현재 홀드 사용권을 소모했는지 여부
}

impl GameInfo {
    // 가방에서 미노를 새로 가져옴.
    pub fn get_mino(&mut self) -> MinoShape {
        // 현재 가방이 비어있거나, 개수가 모자란다면 충전
        self.manage_bag();
        let mino = self.bag.pop_front().unwrap();
        self.manage_bag();
        mino
    }

    pub fn manage_bag(&mut self) {
        if self.bag.len() <= self.next_count as usize {
            self.fill_bag();
        }
    }

    // 현재 가방 채움
    fn fill_bag(&mut self) -> Option<()> {
        match self.bag_mode {
            BagType::SevenBag => {
                let mut new_bag = random::shuffle(&self.mino_list).collect();
                self.bag.append(&mut new_bag);
            }
            BagType::NoBag => {
                let mut new_bag = (0..self.mino_list.len())
                    .map(|_| random::random_select(&self.mino_list))
                    .collect();
                self.bag.append(&mut new_bag);
            }
        }

        Some(())
    }

    fn clear_line(&mut self) -> ClearInfo {
        let mut line = 0;
        // 스핀 여부 반환
        // 지운 줄 수 반환
        for y in (0..self.tetris_board.row_count).into_iter() {
            let row: &Vec<TetrisCell> = &self.tetris_board.cells[y as usize];

            if row.iter().all(|e| e != &TetrisCell::Empty) {
                line += 1;
                for e in (0..=y).into_iter().rev() {
                    if e == 0 {
                        for cell in &mut self.tetris_board.cells[e as usize] {
                            *cell = TetrisCell::Empty
                        }
                    } else {
                        self.tetris_board.cells[e as usize] =
                            self.tetris_board.cells[(e - 1) as usize].clone()
                    }
                }
            }
        }

        let is_perfect = self.tetris_board.unfold().iter().all(|e| e == &0);

        if is_perfect {
            self.record.score += 1000;
            self.record.perfect_clear += 1;
        }

        match line {
            1 => self.record.score += 10,
            2 => self.record.score += 20,
            3 => self.record.score += 30,
            4 => {
                self.record.score += 100;
                self.record.quad += 1;
            }
            _ => {}
        }

        ClearInfo {
            line,
            spin: SpinType::None,
            is_perfect,
        }
    }

    fn fix_current_mino(&mut self) {
        if let Some(current_mino) = self.current_mino {
            // 블럭 고정 후 현재 미노에서 제거
            self.tetris_board
                .write_current_mino(current_mino.cells, self.current_position);
            self.current_mino = None;

            self.hold_used = false;
        }
    }

    pub fn tick(&mut self) {
        if !self.on_play {
            return;
        }

        let current_mino = self.current_mino;

        match current_mino {
            Some(current_mino) => {
                let current_position = self.current_position;
                let next_position = current_position.add_y(1);

                if !valid_mino(&self.tetris_board, &current_mino.cells, next_position) {
                    // 블럭 고정 후 현재 미노에서 제거
                    self.fix_current_mino();
                    self.clear_line();
                } else {
                    self.current_position = next_position;
                }
            }
            None => {
                let mino = self.get_mino();
                self.current_mino = Some(mino);

                let point = Point::start_point(self.tetris_board.column_count);
                self.current_position = point;

                if !valid_mino(&self.tetris_board, &mino.cells, point) {
                    // 패배 처리
                    log::info!("game over");
                    self.on_play = false;
                    self.lose = true;
                    self.current_mino = None;
                }
            }
        }
    }

    pub fn left_move(&mut self) {
        if let Some(current_mino) = self.current_mino {
            let next_position = self.current_position.clone().add_x(-1);

            if valid_mino(&self.tetris_board, &current_mino.cells, next_position) {
                self.current_position = next_position;
            }
        }
    }

    pub fn right_move(&mut self) {
        if let Some(current_mino) = self.current_mino {
            let next_position = self.current_position.clone().add_x(1);

            if valid_mino(&self.tetris_board, &current_mino.cells, next_position) {
                self.current_position = next_position;
            }
        }
    }

    pub fn left_rotate(&mut self) {
        if let Some(current_mino) = &mut self.current_mino {
            if current_mino.mino == Mino::O {
                return;
            }

            let real_length = if current_mino.mino == Mino::I { 4 } else { 3 };
            let mut next_shape = current_mino.cells.clone();

            rotate_left(&mut next_shape, real_length);
            if valid_mino(&self.tetris_board, &next_shape, self.current_position) {
                current_mino.rotation_count = (current_mino.rotation_count + 3) % 4;
                current_mino.cells = next_shape;
            } else {
                for i in 0..4 {
                    let mut next_position = self.current_position.clone();
                    if real_length == 3 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_3BY3[4 + current_mino.rotation_count][i][0],
                            KICK_INDEX_3BY3[4 + current_mino.rotation_count][i][1],
                        ); // 4, 5, 6, 7 => 03, 10, 21, 32
                    } else if real_length == 4 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_I[4 + current_mino.rotation_count][i][0],
                            KICK_INDEX_I[4 + current_mino.rotation_count][i][1],
                        );
                    }
                    if valid_mino(&self.tetris_board, &next_shape, next_position) {
                        current_mino.rotation_count = (current_mino.rotation_count + 3) % 4;
                        self.current_position = next_position;
                        current_mino.cells = next_shape;
                        break;
                    }
                }
            }
        }
    }

    pub fn right_rotate(&mut self) {
        if let Some(current_mino) = &mut self.current_mino {
            if current_mino.mino == Mino::O {
                return;
            }

            let real_length = if current_mino.mino == Mino::I { 4 } else { 3 };

            let mut next_shape = current_mino.cells.clone();
            rotate_right(&mut next_shape, real_length);
            if valid_mino(&self.tetris_board, &next_shape, self.current_position) {
                current_mino.rotation_count = (current_mino.rotation_count + 1) % 4;
                current_mino.cells = next_shape;
            } else {
                for i in 0..4 {
                    let mut next_position = self.current_position.clone();
                    if real_length == 3 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_3BY3[0 + current_mino.rotation_count][i][0],
                            KICK_INDEX_3BY3[0 + current_mino.rotation_count][i][1],
                        ); // 0, 1, 2, 3 => 01, 12, 23, 30
                    } else if real_length == 4 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_I[0 + current_mino.rotation_count][i][0],
                            KICK_INDEX_I[0 + current_mino.rotation_count][i][1],
                        );
                    }
                    if valid_mino(&self.tetris_board, &next_shape, next_position) {
                        current_mino.rotation_count = (current_mino.rotation_count + 1) % 4;
                        self.current_position = next_position;
                        current_mino.cells = next_shape;
                        break;
                    }
                }
            }
        }
    }

    pub fn soft_drop(&mut self) {
        self.tick();
    }

    pub fn get_hard_drop_position(&self) -> Option<Point> {
        match self.current_mino {
            Some(current_mino) => {
                let current_position = self.current_position;
                let mut next_position = current_position.add_y(1);
                loop {
                    if !valid_mino(&self.tetris_board, &current_mino.cells, next_position) {
                        next_position = next_position.add_y(-1);
                        break;
                    } else {
                        next_position = next_position.add_y(1);
                    }
                }

                Some(next_position)
            }
            None => None,
        }
    }

    pub fn hard_drop(&mut self) {
        let position = self.get_hard_drop_position();

        match position {
            Some(position) => {
                self.current_position = position;

                self.fix_current_mino();

                self.clear_line();

                self.tick();
            }
            None => {}
        }
    }

    pub fn hold(&mut self) {
        if !self.hold_used {
            match self.hold {
                Some(hold) => {
                    let temp = self.current_mino;
                    self.current_mino = Some(hold);
                    self.hold = temp;
                }
                None => {
                    self.hold = self.current_mino;
                    self.current_mino = None;
                    self.fill_bag();
                }
            }

            self.hold_used = true;

            self.tick();
        }
    }

    pub fn double_rotate(&mut self) {
        if let Some(current_mino) = &mut self.current_mino {
            if current_mino.mino == Mino::O {
                return;
            }

            let real_length = if current_mino.mino == Mino::I { 4 } else { 3 };

            let mut next_shape = current_mino.cells.clone();
            rotate_right(&mut next_shape, real_length);
            rotate_right(&mut next_shape, real_length);

            if valid_mino(
                &self.tetris_board,
                &current_mino.cells,
                self.current_position,
            ) {
                current_mino.cells = next_shape;
            }
        }
    }
}
