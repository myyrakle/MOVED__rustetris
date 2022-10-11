use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use futures_util::stream::StreamExt;
use gloo_timers::future::IntervalStream;
use wasm_bindgen_futures::spawn_local;

use crate::game::game_info::GameInfo;
use crate::game::tetris_board::TetrisBoard;
use crate::game::tetris_cell::TetrisCell;
use crate::minos::shapes::MinoShape;
use crate::options::game_option::GameOption;
use crate::wasm_bind;

pub struct GameManager {
    pub game_info: Arc<Mutex<GameInfo>>,
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

        let mino_list = vec![
            MinoShape::I,
            MinoShape::L,
            MinoShape::J,
            MinoShape::S,
            MinoShape::Z,
            MinoShape::O,
            MinoShape::T,
        ];

        let game_info = GameInfo {
            record: Default::default(),
            render_interval: 200,
            tick_interval: 2000,
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

        let game_info = Arc::new(Mutex::new(game_info));

        Self { game_info }
    }

    pub fn on_play(&self) -> bool {
        self.game_info.lock().unwrap().on_play
    }

    pub fn start_game(&self) -> Option<()> {
        if self.on_play() {
            return None;
        }

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

                game_info.tick();
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

                if game_info.on_play {
                    let tetris_board = match game_info.current_mino {
                        Some(current_mino) => {
                            let mut tetris_board = game_info.tetris_board.clone();
                            tetris_board
                                .write_current_mino(current_mino, game_info.current_position);

                            tetris_board
                        }
                        None => game_info.tetris_board.clone(),
                    };

                    wasm_bind::render(
                        tetris_board.unfold(),
                        tetris_board.board_width,
                        tetris_board.board_height,
                        tetris_board.column_count,
                        tetris_board.row_count,
                    );
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
