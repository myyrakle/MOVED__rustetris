use std::collections::VecDeque;

use crate::game::{
    valid_mino, valid_tspin, BagType, ClearInfo, GameRecord, MinoShape, Point, SpinType,
    TetrisBoard, TetrisCell,
};

use crate::js_bind::write_text::write_text;
use crate::options::game_option::GameOption;
use crate::util::{random, rotate_left, rotate_right, KICK_INDEX_3BY3, KICK_INDEX_I};

use super::{calculate_score, Mino};

#[derive(Debug)]
pub struct GameInfo {
    pub record: GameRecord,

    pub running_time: u128, // 실행시간 (밀리초)

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

    pub bag_mode: BagType, //가방 순환 규칙 사용여부 (false면 완전 랜덤. true면 한 묶음에서 랜덤)
    pub mino_list: Vec<MinoShape>, //미노 리스트

    pub hold: Option<MinoShape>, // 홀드한 미노
    pub hold_used: bool,         // 현재 홀드 사용권을 소모했는지 여부

    pub combo: Option<u32>, // 현재 콤보 (제로콤보는 None, 지웠을 경우 0부터 시작)
    pub back2back: Option<u32>, // 현재 백투백 스택 (제로는 None, 지웠을 경우 0부터 시작)

    pub message: Option<String>, // 렌더링할 메세지

    pub in_spin: SpinType, // 현재 스핀 상태 확인

    pub lock_delay: u32, // 바닥에 닿을때 고정하기까지의 딜레이. 밀리초 단위.
    pub lock_delay_count: u8, // 하좌우이동, 좌우회전 성공 시 록딜레이 카운트가 올라감. 틱스레드에서 변화를 읽고 start를 초기화. 8이상이면 안올라감

    pub sdf: u32, // soft drop fast. 소프트 드랍 속도
    pub das: u32, // delay auto shift. 밀리초 단위.
    pub arr: u32, // auto repeat shift. 좌우 이동 클릭시,
}

impl GameInfo {
    pub fn with_option(option: GameOption) -> Self {
        let column_count = option.column_count;
        let hidden_row_count = 4;
        let row_count = option.row_count + hidden_row_count;
        let board_height = option.board_height;
        let board_width = option.board_width;
        let bag_mode = option.bag_mode;
        let tetris_board = TetrisBoard {
            cells: vec![
                vec![TetrisCell::Empty; column_count as usize];
                row_count as usize + hidden_row_count as usize
            ],
            column_count,
            row_count,
            board_height,
            board_width,
            hidden_row_count,
        };

        let mino_list = vec![
            MinoShape::I,
            MinoShape::L,
            MinoShape::J,
            MinoShape::S,
            MinoShape::Z,
            MinoShape::O,
            MinoShape::T,
        ];

        Self {
            record: Default::default(),
            render_interval: 200,
            tick_interval: 1000,
            current_position: Default::default(),
            current_mino: None,
            freezed: false,
            next_count: 5,
            bag: VecDeque::new(),
            tetris_board,
            on_play: false,
            lose: false,
            bag_mode,
            mino_list,
            hold: None,
            hold_used: false,
            back2back: None,
            combo: None,
            message: None,
            in_spin: SpinType::None,
            lock_delay: 500,
            das: 300, //미사용
            sdf: 0,   //미사용
            arr: 0,   //미사용
            running_time: 0,
            lock_delay_count: 0,  
        }
    }

    // 가방에서 미노를 새로 가져옴.
    pub fn get_mino(&mut self) -> MinoShape {
        // 현재 가방이 비어있거나, 개수가 모자란다면 충전
        self.manage_bag();
        let mino = self.bag.pop_front().unwrap();
        self.manage_bag();
        mino
    }

    // 가방이 비어있을 경우 충전
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

    // 지울 줄이 있을 경우 줄을 지움
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

        if line > 0 {
            let mut is_back2back = false;

            match self.combo {
                Some(combo) => {
                    self.combo = Some(combo + 1);

                    match line {
                        1..=3 => {
                            self.message = None;
                        }
                        4 => {
                            self.message = Some("Quad".into());
                        }
                        _ => {}
                    }
                }
                None => {
                    self.combo = Some(0);

                    match line {
                        1..=3 => {
                            self.message = None;
                        }
                        4 => {
                            self.message = Some("Quad".into());
                            self.record.quad += 1;
                            is_back2back = true
                        }
                        _ => {}
                    }
                }
            }

            match self.in_spin.clone() {
                SpinType::TSpin => {
                    is_back2back = true;

                    match line {
                        1 => self.message = Some("T-Spin Single".into()),
                        2 => self.message = Some("T-Spin Double".into()),
                        3 => self.message = Some("T-Spin Triple".into()),
                        _ => {}
                    }
                }
                SpinType::Spin => {}
                SpinType::Mini => {
                    is_back2back = true;

                    match line {
                        1 => self.message = Some("T-Spin Single mini".into()),
                        2 => self.message = Some("T-Spin Double mini".into()),
                        3 => self.message = Some("T-Spin Triple mini".into()),
                        _ => {}
                    }
                }
                SpinType::None => {}
            }

            if is_back2back {
                match self.back2back {
                    Some(back2back) => {
                        self.back2back = Some(back2back + 1);
                    }
                    None => {
                        self.back2back = Some(0);
                    }
                }
            } else {
                self.back2back = None;
            }

            if is_perfect {
                self.record.perfect_clear += 1;
                self.message = Some("Perfect Clear".into())
            }
        } else {
            self.combo = None;
        }

        let score = calculate_score(
            line,
            is_perfect,
            self.combo,
            self.in_spin.clone(),
            self.back2back,
        );
        self.record.score += score;

        self.after_clear();

        ClearInfo {
            line,
            spin: SpinType::None,
            is_perfect,
        }
    }

    // 현재 미노를 고정
    fn fix_current_mino(&mut self) {
        if let Some(current_mino) = self.current_mino {
            // 블럭 고정 후 현재 미노에서 제거
            self.tetris_board
                .write_current_mino(current_mino.cells, self.current_position);
            self.current_mino = None;
            self.lock_delay_count = 0;

            self.hold_used = false;
        }
    }

    // clear 처리 후에 트리거 (줄이 지워지는지 여부와 별개)
    fn after_clear(&mut self) {
        self.in_spin = SpinType::None;
    }

    // 한칸 내려간 후에 트리거
    fn after_down(&mut self) {
        self.in_spin = SpinType::None;
    }

    // 한칸씩 아래로 내려가는 중력 동작
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
                    self.fix_current_mino();
                    self.clear_line();
                } else {
                    self.current_position = next_position;
                    self.after_down();
                }
            }
            None => {
                let mino = self.get_mino();
                self.current_mino = Some(mino);

                let point = Point::start_point(self.tetris_board.column_count);
                self.current_position = point;

                if !valid_mino(&self.tetris_board, &mino.cells, point) {
                    // 패배 처리
                    self.game_over();
                }
            }
        }
    }

    // 왼쪽 이동
    pub fn left_move(&mut self) {
        if let Some(current_mino) = self.current_mino {
            let next_position = self.current_position.clone().add_x(-1);

            if valid_mino(&self.tetris_board, &current_mino.cells, next_position) {
                self.current_position = next_position;
                if !valid_mino(&self.tetris_board, &current_mino.cells, self.current_position.add_y(1)) { 
                    self.lock_delay_count += 1;
                }
            }
        }
    }

    // 오른쪽 이동
    pub fn right_move(&mut self) {
        if let Some(current_mino) = self.current_mino {
            let next_position = self.current_position.clone().add_x(1);

            if valid_mino(&self.tetris_board, &current_mino.cells, next_position) {
                self.current_position = next_position; 
                if !valid_mino(&self.tetris_board, &current_mino.cells, self.current_position.add_y(1)) { 
                    self.lock_delay_count += 1;
                }
            }
        }
    }

    // 왼쪽 회전 (반시계방향)
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
                if !valid_mino(&self.tetris_board, &current_mino.cells, self.current_position.add_y(1)){ 
                    self.lock_delay_count += 1;
                }

                if current_mino.mino == Mino::T {
                    self.in_spin =
                        valid_tspin(&self.tetris_board, &current_mino, self.current_position, 0);
                }
            } else {
                for i in 0..4 {
                    let mut next_position = self.current_position.clone();
                    if real_length == 3 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_3BY3[4 + current_mino.rotation_count][i][0],
                            -KICK_INDEX_3BY3[4 + current_mino.rotation_count][i][1],
                        ); // 4, 5, 6, 7 => 03, 10, 21, 32
                    } else if real_length == 4 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_I[4 + current_mino.rotation_count][i][0],
                            -KICK_INDEX_I[4 + current_mino.rotation_count][i][1],
                        );
                    }
                    if valid_mino(&self.tetris_board, &next_shape, next_position) {
                        current_mino.rotation_count = (current_mino.rotation_count + 3) % 4;
                        self.current_position = next_position;
                        current_mino.cells = next_shape;
                        if !valid_mino(&self.tetris_board, &current_mino.cells, self.current_position.add_y(1)){ 
                            self.lock_delay_count += 1;
                        }
                

                        if current_mino.mino == Mino::T {
                            self.in_spin =
                                valid_tspin(&self.tetris_board, &current_mino, next_position, i);
                        }

                        break;
                    }
                }
            }
        }
    }

    // 오른쪽 회전 (시계방향)
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
                if !valid_mino(&self.tetris_board, &current_mino.cells, self.current_position.add_y(1)){ 
                    self.lock_delay_count += 1;
                }

                if current_mino.mino == Mino::T {
                    self.in_spin =
                        valid_tspin(&self.tetris_board, &current_mino, self.current_position, 0);
                }
            } else {
                for i in 0..4 {
                    let mut next_position = self.current_position.clone();
                    if real_length == 3 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_3BY3[0 + current_mino.rotation_count][i][0],
                            -KICK_INDEX_3BY3[0 + current_mino.rotation_count][i][1],
                        ); // 0, 1, 2, 3 => 01, 12, 23, 30
                    } else if real_length == 4 {
                        next_position = next_position.move_xy(
                            KICK_INDEX_I[0 + current_mino.rotation_count][i][0],
                            -KICK_INDEX_I[0 + current_mino.rotation_count][i][1],
                        );
                    }
                    if valid_mino(&self.tetris_board, &next_shape, next_position) {
                        current_mino.rotation_count = (current_mino.rotation_count + 1) % 4;
                        self.current_position = next_position;
                        current_mino.cells = next_shape;
                        if !valid_mino(&self.tetris_board, &current_mino.cells, self.current_position.add_y(1)){ 
                            self.lock_delay_count += 1;
                        }
                        if current_mino.mino == Mino::T {
                            self.in_spin =
                                valid_tspin(&self.tetris_board, &current_mino, next_position, i);
                        }

                        break;
                    }
                }
            }
        }
    }

    // 소프트드랍
    pub fn soft_drop(&mut self) {
        self.tick();
    }

    // 하드드랍될 위치 획득
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

    // 하드드랍 동작
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

    // 미노 홀드
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

    // 180도 회전
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

    // 게임오버
    fn game_over(&mut self) {
        self.on_play = false;
        self.lose = true;
        self.current_mino = None;
        write_text("message", "Game Over".into());
    }

    // 보드 초기화
    pub fn init_board(&mut self) -> Option<()> {
        let column_count = self.tetris_board.column_count;
        let row_count = self.tetris_board.row_count;

        self.tetris_board = TetrisBoard {
            cells: vec![vec![TetrisCell::Empty; column_count as usize]; row_count as usize],
            row_count,
            column_count,
            board_height: self.tetris_board.board_height,
            board_width: self.tetris_board.board_width,
            hidden_row_count: self.tetris_board.hidden_row_count,
        };

        Some(())
    }

    // 컨텍스트 초기화
    pub fn init_context(&mut self) -> Option<()> {
        self.back2back = None;
        self.combo = None;
        self.in_spin = SpinType::None;
        self.message = None;

        Some(())
    }

    // 가방 초기화
    pub fn init_bag(&mut self) -> Option<()> {
        self.bag = VecDeque::new();
        self.current_mino = None;
        self.hold_used = false;
        self.hold = None;

        Some(())
    }

    // 점수 초기화
    pub fn init_score(&mut self) -> Option<()> {
        self.record = Default::default();

        Some(())
    }

    pub fn init_running_time(&mut self) -> Option<()> {
        self.running_time = 0;
        Some(())
    }

    // 게임 초기화
    pub fn init_game(&mut self) -> Option<()> {
        self.init_bag()?;
        self.init_board()?;
        self.init_score()?;
        self.init_context()?;
        self.init_running_time()?;

        Some(())
    }
}
