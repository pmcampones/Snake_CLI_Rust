use std::{thread, time, process::Command, io};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;
use crate::display_renderer::DisplayRenderer;
use crate::Snake;
use crate::snake::Movable;

const FRAME_INTERVAL_MILIS : Duration = time::Duration::from_millis(100);

pub(crate) fn play(mut snake : Snake, mut renderer: DisplayRenderer, rx : Receiver<char>) {
    let mut prev_displacement = (1, 0);
    loop {
        //Command::new("clear").spawn().unwrap();
        let displacement = compute_displacement(&rx, &mut prev_displacement);
        snake.mv(displacement);
        renderer.next_frame(&snake);
        thread::sleep(FRAME_INTERVAL_MILIS);
        prev_displacement = displacement;
    }
}

fn compute_displacement(rx: & Receiver<char>, prev: &mut (isize, isize)) -> (isize, isize) {
    let displacement = match rx.try_recv() {
        Ok('w') => compute_axis_displacement(prev, (0, -1), prev.1),
        Ok('a') => compute_axis_displacement(prev, (-1, 0), prev.0),
        Ok('s') => compute_axis_displacement(prev, (0, 1), prev.1),
        Ok('d') => compute_axis_displacement(prev, (1, 0), prev.0),
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