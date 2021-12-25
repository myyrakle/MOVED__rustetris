use std::collections::VecDeque;

use yew::prelude::*;

use crate::functions::random;
use crate::minos::shapes::{MinoType, I, J, L, O, S, T, Z};
use crate::types::tetris_cell::TetrisCell;
use crate::{options::game_option::GameOption, types::point::Point};
use rand::seq::SliceRandom;

pub enum Msg {
    AddOne,
}

pub struct Model {
    column_count: u8,                   //테트리스 열 개수(가로 길이)
    row_count: u8,                      //테트리스 행 개수(세로 길이)
    tetris_board: Vec<Vec<TetrisCell>>, //테트리스 보드
    bag_mode: bool, //가방 순환 규칙 사용여부 (false면 완전 랜덤. true면 한 묶음에서 랜덤)

    game_score: u64, //게임 점수

    current_position: Option<Point>, //현재 미노 위치
    freezed: bool,                   //현재 미노가 보드에 붙었는지?

    mino_list: Vec<MinoType>, //미노 리스트

    current_bag: VecDeque<MinoType>, //현재 가방
    next_bag: VecDeque<MinoType>,    //다음 가방
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

        Self {
            column_count,
            row_count,
            tetris_board,
            bag_mode,
            game_score: 0,
            current_position: None,
            freezed: false,
            mino_list,
            current_bag: VecDeque::new(),
            next_bag: VecDeque::new(),
        }
    }

    // 가방에서 미노를 새로 가져옴.
    pub fn get_mino(&mut self) -> MinoType {
        // 현재 가방이 비어있다면
        if self.current_bag.is_empty() {
            self.fill_current_bag();
        }

        self.current_bag.pop_front().unwrap()
    }

    // 현재 가방 채움
    fn fill_current_bag(&mut self) {
        if self.next_bag.is_empty() {
            self.fill_next_bag();
        }

        self.current_bag = self.next_bag.clone();
        self.fill_next_bag();
    }

    // 다음 가방 채움
    fn fill_next_bag(&mut self) {
        if self.bag_mode {
            self.next_bag = random::shuffle(&self.mino_list).collect();
        } else {
            self.next_bag = (0..self.mino_list.len())
                .map(|_| random::random_select(&self.mino_list))
                .collect()
        }
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
            </span>
        }
    }
}
