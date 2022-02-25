use std::{thread, time};
use std::time::Duration;
use crate::display_renderer::DisplayRenderer;
use crate::Snake;
use crate::snake::Movable;

const FRAME_INTERVAL_MILIS : Duration = time::Duration::from_millis(250);

pub(crate) fn play(mut snake : Snake, mut renderer: DisplayRenderer) {
    loop {
        snake.mv((1,0));
        renderer.next_frame(&snake);
        thread::sleep(FRAME_INTERVAL_MILIS);
    }
}