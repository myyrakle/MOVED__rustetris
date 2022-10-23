use futures_util::stream::StreamExt;
use gloo_timers::future::IntervalStream;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::spawn_local;

use crate::constants::character::SPECIAL_SPACE;
use crate::constants::time::TICK_LOOP_INTERVAL;
use crate::game::game_info::GameInfo;
use crate::js_bind::request_animation_frame::request_animation_frame;
use crate::js_bind::write_text::write_text;
use crate::options::game_option::GameOption;
use crate::wasm_bind;

pub struct GameManager {
    pub game_info: Arc<Mutex<GameInfo>>,
}

impl GameManager {
    pub fn empty_render() {
        let manager = Self::new();

        let game_info = manager.game_info.lock().unwrap();
        let tetris_board = game_info.tetris_board.clone();

        wasm_bind::render_board(
            tetris_board.unfold(),
            tetris_board.board_width,
            tetris_board.board_height,
            tetris_board.column_count,
            tetris_board.row_count,
            tetris_board.hidden_row_count,
        );

        let next = game_info.bag.iter().map(|e| e.mino.into()).collect();
        wasm_bind::render_next(next, 120, 520, 6, 26);

        wasm_bind::render_hold(game_info.hold.map(|e| e.mino.into()), 120, 120, 6, 6);
    }

    pub fn new() -> Self {
        Self::with_option(Default::default())
    }

    pub fn with_option(option: GameOption) -> Self {
        let game_info = GameInfo::with_option(option);

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

        self.game_info.lock().ok()?.on_play = true;
        self.game_info.lock().ok()?.lose = false;

        log::info!("GAME START");

        // tick - 중력 스레드
        let game_info = Arc::clone(&self.game_info);
        let mut former_lock_delay_count:u8 = 0;
        spawn_local(async move {
            // 시작 기준점
            let mut start_point = instant::Instant::now();

            let game_info = game_info;
            let _game_info = Arc::clone(&game_info);

            // 기본 100밀리초 단위마다 반복해서 타임 체크 (더 세밀한 제어가 필요하다면 문제없는 선에서 낮춰도 무방)
            let mut future_list = IntervalStream::new(TICK_LOOP_INTERVAL).map(move |_| {
                let mut game_info = game_info.lock().unwrap();
                if former_lock_delay_count != game_info.lock_delay_count{
                    if game_info.lock_delay_count<8 {
                        start_point = instant::Instant::now();
                    }
                    former_lock_delay_count = game_info.lock_delay_count;
                }
                game_info.running_time += TICK_LOOP_INTERVAL as u128;

                let duration = start_point.elapsed();

                // tick이 발생하지 않은 시점에서 경과된 시간.
                let elapsed_time = duration.as_millis();

                // 여기서 딜레이 커스텀하면 될듯
                let delay = game_info.tick_interval as u128 + (game_info.lock_delay as u128);

                // 지정된 딜레이만큼 지났다면 다시 초기화하고 tick 한칸 수행
                if elapsed_time >= delay {
                    start_point = instant::Instant::now();
                    game_info.tick();
                }
            });

            let game_info = _game_info;
            loop {
                if game_info.lock().unwrap().on_play {
                    let next = future_list.next();
                    next.await;
                } else {
                    break;
                }
            }
        });

        // 렌더링 스레드
        let game_info = Arc::clone(&self.game_info);
        spawn_local(async move {
            let f = Rc::new(RefCell::new(None));
            let g = f.clone();

            *g.borrow_mut() = Some(Closure::new(move || {
                let game_info = game_info.lock().unwrap();

                if !game_info.on_play {
                    // Drop our handle to this closure so that it will get cleaned
                    // up once we return.
                    let _ = f.borrow_mut().take();
                    return;
                }

                let tetris_board = match game_info.current_mino {
                    Some(current_mino) => {
                        let mut tetris_board = game_info.tetris_board.clone();
                        tetris_board
                            .write_current_mino(current_mino.cells, game_info.current_position);

                        let ghost_position = game_info.get_hard_drop_position().unwrap();
                        tetris_board.write_current_mino(
                            current_mino.clone().to_ghost().cells,
                            ghost_position,
                        );

                        tetris_board
                    }
                    None => game_info.tetris_board.clone(),
                };

                wasm_bind::render_board(
                    tetris_board.unfold(),
                    tetris_board.board_width,
                    tetris_board.board_height,
                    tetris_board.column_count,
                    tetris_board.row_count,
                    tetris_board.hidden_row_count,
                );

                let next = game_info.bag.iter().map(|e| e.mino.into()).collect();
                wasm_bind::render_next(next, 120, 520, 6, 26);

                wasm_bind::render_hold(game_info.hold.map(|e| e.mino.into()), 120, 120, 6, 6);

                write_text("score", game_info.record.score.to_string());
                write_text("pc", game_info.record.perfect_clear.to_string());
                write_text("quad", game_info.record.quad.to_string());

                if let Some(back2back) = game_info.back2back {
                    if back2back != 0 {
                        write_text("back2back", format!("Back2Back {}", back2back));
                    }
                } else {
                    write_text("back2back", SPECIAL_SPACE.into());
                }

                if let Some(combo) = game_info.combo {
                    if combo > 0 {
                        write_text("combo", format!("Combo {}", combo));
                    }
                } else {
                    write_text("combo", SPECIAL_SPACE.into());
                }

                if let Some(message) = game_info.message.clone() {
                    write_text("message", message);
                } else {
                    write_text("message", SPECIAL_SPACE.into());
                }

                request_animation_frame(f.borrow().as_ref().unwrap());
            }));

            request_animation_frame(g.borrow().as_ref().unwrap());
        });

        Some(())
    }

    pub fn end_game(&self) -> Option<()> {
        self.game_info.lock().ok()?.on_play = false;

        Some(())
    }


    pub fn init_running_time(&self) -> Option<()> {
        let mut game_info = self.game_info.lock().ok().unwrap();
        game_info.running_time = 0;
        Some(())
    }


    // 보드 초기화

    // 컨텍스트 초기화


    // 가방 초기화


    // 점수 초기화

}
