use std::collections::VecDeque;
use std::error::Error;
use std::sync::{Arc, Mutex, MutexGuard};

use yew::prelude::*;

use crate::functions::random;
use crate::minos::shapes::{MinoType, I, J, L, O, S, T, Z};
use crate::types::game_info::GameInfo;
use crate::types::tetris_cell::TetrisCell;
use crate::{options::game_option::GameOption, types::point::Point};

pub enum Msg {
    AddOne,
}

pub struct Model {
    column_count: u8, //테트리스 열 개수(가로 길이)
    row_count: u8,    //테트리스 행 개수(세로 길이)
    bag_mode: bool,   //가방 순환 규칙 사용여부 (false면 완전 랜덤. true면 한 묶음에서 랜덤)

    mino_list: Vec<MinoType>, //미노 리스트

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
        let tetris_board = vec![vec![TetrisCell::Empty; column_count as usize]; row_count as usize];

        let mino_list = vec![I, L, J, S, Z, O, T];

        let game_info = GameInfo {
            game_score: 0,
            render_interval: 30,
            tick_interval: 1000,
            current_position: None,
            current_mino: None,
            freezed: false,
            current_bag: VecDeque::new(),
            next_bag: VecDeque::new(),
            tetris_board,
            on_play: false,
            lose: false,
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

        let game_info = Arc::clone(&self.game_info);
        std::thread::spawn(move || loop {
            let time = std::time::Duration::from_millis(game_info.lock().unwrap().tick_interval);

            if game_info.lock().unwrap().on_play {
            } else {
                break;
            }

            std::thread::sleep(time);
        });

        let game_info = Arc::clone(&self.game_info);
        std::thread::spawn(move || loop {
            let time = std::time::Duration::from_millis(game_info.lock().unwrap().render_interval);

            if game_info.lock().unwrap().on_play {
            } else {
                break;
            }

            std::thread::sleep(time);
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
        game_info.tetris_board =
            vec![vec![TetrisCell::Empty; self.column_count as usize]; self.row_count as usize];

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
    pub fn get_mino(&self) -> Result<MinoType, Box<dyn Error>> {
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
            Msg::AddOne => {
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <span>
                <canvas id="gamebox" width="300" height="600"></canvas>
                <button>{"Start"}</button>
            </span>
        }
    }
}
