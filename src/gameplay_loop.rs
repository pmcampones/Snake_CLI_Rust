use std::{thread, time, process::Command, io};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;
use crate::display_renderer::DisplayRenderer;
use crate::Snake;
use crate::snake::Movable;

const FRAME_INTERVAL_MILIS : Duration = time::Duration::from_millis(100);

const LFT_MV : (isize, isize) = (-1, 0);
const RGT_MV : (isize, isize) = (1, 0);
const UP_MV  : (isize, isize) = (0, -1);
const DWN_MV : (isize, isize) = (0, 1);

const LFT_IN : char = 'a';
const RGT_IN : char = 'd';
const UP_IN  : char = 'w';
const DWN_IN : char = 's';

pub(crate) fn play(mut snake : Snake, mut renderer: DisplayRenderer, rx : Receiver<char>) {
    let mut prev_displacement = (1, 0);
    loop {
        //Command::new("clear").spawn().unwrap();
        let displacement = compute_displacement(&rx, &mut prev_displacement);
        if !renderer.is_in_wall(snake.get_pos()) {
            panic!("Stop hitting the wall asshole!!")
        } else if snake.is_eating_self() {
            panic!("Eating yourself?... Kinky ;)")
        }
        snake.mv(displacement);
        renderer.next_frame(&snake);
        thread::sleep(FRAME_INTERVAL_MILIS);
        prev_displacement = displacement;
    }
}

fn compute_displacement(rx: &Receiver<char>, prev: &mut (isize, isize)) -> (isize, isize) {
    let displacement = match rx.try_recv() {
        Ok(LFT_IN) => compute_axis_displacement(prev, LFT_MV, prev.0),
        Ok(RGT_IN) => compute_axis_displacement(prev, RGT_MV, prev.0),
        Ok(UP_IN) => compute_axis_displacement(prev, UP_MV, prev.1),
        Ok(DWN_IN) => compute_axis_displacement(prev, DWN_MV, prev.1),
        _ => *prev
    };
    displacement
}

fn compute_axis_displacement(
    prev: &mut (isize, isize), proposal: (isize, isize), axis: isize) -> (isize, isize) {
    if axis == 0 {
        proposal
    } else {
        *prev
    }
}