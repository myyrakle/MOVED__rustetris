use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use futures_util::stream::StreamExt;
use gloo_timers::future::IntervalStream;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::game::game_info::GameInfo;
use crate::game::point::Point;
use crate::game::tetris_board::TetrisBoard;
use crate::game::tetris_cell::TetrisCell;
use crate::minos::shapes::{I, J, L, O, S, T, Z};
use crate::options::game_option::GameOption;
use crate::util::valid_mino::valid_mino;
use crate::wasm_bind;

pub struct GameManager {
    game_info: Arc<Mutex<GameInfo>>,
}

impl GameManager {
    pub fn new() -> Self {
        Self::with_option(Default::default())
    }

    pub fn with_option(option: GameOption) -> Self {
        let column_count = option.column_count;
        let row_count = option.row_count;
        let board_height = option.board_height;
        let board_width = option.board_width;
        let bag_mode = option.bag_mode;
        let tetris_board = TetrisBoard {
            cells: vec![vec![TetrisCell::Empty; column_count as usize]; row_count as usize],
            column_count,
            row_count,
            board_height,
            board_width,
        };

        let mino_list = vec![I, L, J, S, Z, O, T];

        let game_info = GameInfo {
            record: Default::default(),
            render_interval: 100,
            tick_interval: 1000,
            current_position: Default::default(),
            current_mino: None,
            freezed: false,
            current_bag: VecDeque::new(),
            next_bag: VecDeque::new(),
            tetris_board,
            on_play: false,
            lose: false,
            tick_interval_handler: None,
            render_interval_handler: None,
            bag_mode,
            mino_list,
        };

        Self {
            game_info: Arc::new(Mutex::new(game_info)),
        }
    }

    pub fn start_game(&self) -> Option<()> {
        self.init_game()?;
        self.game_info.lock().ok()?.on_play = true;
        self.game_info.lock().ok()?.lose = false;

        log::info!("GAME START");

        // 틱 스레드
        let game_info = Arc::clone(&self.game_info);
        spawn_local(async move {
            let game_info = game_info;

            let tick_interval = game_info.lock().ok().unwrap().tick_interval;

            let mut future_list = IntervalStream::new(tick_interval as u32).map(move |_| {
                //log::info!("TICK");

                let mut game_info = game_info.lock().unwrap();

                let current_mino = game_info.current_mino;

                match current_mino {
                    Some(current_mino) => {
                        //current_mino;
                        let current_position = game_info.current_position;
                        let next_position =
                            current_position.set_y(game_info.current_position.y + 1);

                        if !valid_mino(&game_info.tetris_board, &current_mino, next_position) {
                            // 블럭 고정 후 현재 미노에서 제거
                            game_info
                                .tetris_board
                                .write_current_mino(current_mino, current_position);
                            game_info.current_mino = None;
                        } else {
                            game_info.current_position = next_position;
                        }

                        // TODO: 줄 지우기
                    }
                    None => {
                        let mino = game_info.get_mino();
                        game_info.current_mino = Some(mino);

                        let point = Point::start_point(game_info.tetris_board.column_count);
                        game_info.current_position = point;

                        if !valid_mino(&game_info.tetris_board, &mino, point) {
                            // 패배 처리
                            game_info.on_play = false;
                            game_info.lose = true;
                        } else {
                        }
                    }
                }
            });

            loop {
                let next = future_list.next();
                next.await;
            }
        });

        // 렌더링 스레드
        let game_info = Arc::clone(&self.game_info);
        spawn_local(async move {
            let game_info = game_info;

            let render_interval = game_info.lock().ok().unwrap().render_interval;

            let mut future_list = IntervalStream::new(render_interval as u32).map(move |_| {
                //log::info!("RENDER");

                let game_info = game_info.lock().unwrap();

                let tetris_board = match game_info.current_mino {
                    Some(current_mino) => {
                        let mut tetris_board = game_info.tetris_board.clone();
                        tetris_board.write_current_mino(current_mino, game_info.current_position);

                        tetris_board
                    }
                    None => game_info.tetris_board.clone(),
                };

                if game_info.on_play {
                    wasm_bind::render(
                        tetris_board.unfold(),
                        game_info.tetris_board.board_width,
                        game_info.tetris_board.board_height,
                        game_info.tetris_board.column_count,
                        game_info.tetris_board.row_count,
                    );
                } else {
                    // NONE
                }
            });

            loop {
                let next = future_list.next();
                next.await;
            }
        });

        Some(())
    }

    pub fn end_game(&self) -> Option<()> {
        self.game_info.lock().ok()?.on_play = false;

        Some(())
    }

    // 게임 초기화
    pub fn init_game(&self) -> Option<()> {
        self.init_bag()?;
        self.init_board()?;
        self.init_score()?;

        Some(())
    }

    // 보드 초기화
    pub fn init_board(&self) -> Option<()> {
        let mut game_info = self.game_info.lock().ok().unwrap();
        let column_count = game_info.tetris_board.column_count;
        let row_count = game_info.tetris_board.row_count;

        game_info.tetris_board = TetrisBoard {
            cells: vec![vec![TetrisCell::Empty; column_count as usize]; row_count as usize],
            row_count,
            column_count,
            board_height: game_info.tetris_board.board_height,
            board_width: game_info.tetris_board.board_width,
        };

        Some(())
    }

    // 가방 초기화
    pub fn init_bag(&self) -> Option<()> {
        let mut game_info = self.game_info.lock().ok().unwrap();

        game_info.current_bag = VecDeque::new();
        game_info.next_bag = VecDeque::new();

        Some(())
    }

    // 점수 초기화
    pub fn init_score(&self) -> Option<()> {
        let mut game_info = self.game_info.lock().ok().unwrap();

        game_info.record = Default::default();

        Some(())
    }
}
