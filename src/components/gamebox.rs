use std::collections::VecDeque;
use std::error::Error;
use std::sync::{Arc, Mutex};

use futures_util::stream::StreamExt;
use gloo_timers::future::IntervalStream;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::functions::random;
use crate::minos::shapes::{MinoShape, I, J, L, O, S, T, Z};
use crate::options::game_option::GameOption;
use crate::types::game_info::GameInfo;
use crate::types::tetris_board::TetrisBoard;
use crate::types::tetris_cell::TetrisCell;
use crate::wasm_bind;

pub enum Msg {
    GameStart,
}

pub struct Model {
    column_count: u8, //테트리스 열 개수(가로 길이)
    row_count: u8,    //테트리스 행 개수(세로 길이)
    bag_mode: bool,   //가방 순환 규칙 사용여부 (false면 완전 랜덤. true면 한 묶음에서 랜덤)

    mino_list: Vec<MinoShape>, //미노 리스트

    game_info: Arc<Mutex<GameInfo>>,
}

impl Model {
    pub fn new() -> Self {
        Self::with_option(GameOption::build())
    }

    pub fn with_option(option: GameOption) -> Self {
        let column_count = option.column_count.unwrap_or(10);
        let row_count = option.row_count.unwrap_or(20);
        let bag_mode = option.bag_mode.unwrap_or(true);
        let tetris_board = TetrisBoard {
            cells: vec![vec![TetrisCell::Empty; column_count as usize]; row_count as usize],
            column_count,
            row_count,
        };

        let mino_list = vec![I, L, J, S, Z, O, T];

        let game_info = GameInfo {
            game_score: 0,
            render_interval: 100,
            tick_interval: 1000,
            current_position: None,
            current_mino: None,
            freezed: false,
            current_bag: VecDeque::new(),
            next_bag: VecDeque::new(),
            tetris_board,
            on_play: false,
            lose: false,
            tick_interval_handler: None,
            render_interval_handler: None,
        };

        Self {
            column_count,
            row_count,
            bag_mode,
            mino_list,
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

                let game_info = game_info.lock().unwrap();

                if game_info.on_play {
                    wasm_bind::render(
                        game_info.tetris_board.unfold(),
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

        // 렌더링 스레드
        let game_info = Arc::clone(&self.game_info);
        spawn_local(async move {
            let game_info = game_info;

            let render_interval = game_info.lock().ok().unwrap().render_interval;

            let mut future_list = IntervalStream::new(render_interval as u32).map(move |_| {
                //log::info!("RENDER");

                let game_info = game_info.lock().unwrap();

                if game_info.on_play {
                    wasm_bind::render(
                        game_info.tetris_board.unfold(),
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
        game_info.tetris_board = TetrisBoard {
            cells: vec![
                vec![TetrisCell::Empty; self.column_count as usize];
                self.row_count as usize
            ],
            row_count: self.row_count,
            column_count: self.column_count,
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

        game_info.game_score = 0;

        Some(())
    }

    // 가방에서 미노를 새로 가져옴.
    pub fn get_mino(&self) -> Result<MinoShape, Box<dyn Error>> {
        let mut game_info = self.game_info.lock().ok().unwrap();

        // 현재 가방이 비어있다면
        if game_info.current_bag.is_empty() {
            self.fill_current_bag();
        }

        Ok(game_info.current_bag.pop_front().unwrap())
    }

    // 현재 가방 채움
    fn fill_current_bag(&self) -> Option<()> {
        let mut game_info = self.game_info.lock().ok().unwrap();

        if game_info.next_bag.is_empty() {
            self.fill_next_bag()?;
        }

        game_info.current_bag = game_info.next_bag.clone();
        self.fill_next_bag()?;

        Some(())
    }

    // 다음 가방 채움
    fn fill_next_bag(&self) -> Option<()> {
        let mut game_info = self.game_info.lock().ok().unwrap();

        if self.bag_mode {
            game_info.next_bag = random::shuffle(&self.mino_list).collect();
        } else {
            game_info.next_bag = (0..self.mino_list.len())
                .map(|_| random::random_select(&self.mino_list))
                .collect()
        }

        Some(())
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::new()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GameStart => {
                self.start_game();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <span>
                <canvas id="gamebox" width="300" height="600"></canvas>
                <button onclick={link.callback(|_| Msg::GameStart)}>{"Start"}</button>
            </span>
        }
    }
}
