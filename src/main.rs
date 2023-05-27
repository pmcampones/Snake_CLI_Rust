use crate::snake::Snake;

use std::{io, thread};
use std::sync::mpsc;
use std::sync::mpsc::Sender;

mod snake;
mod display_renderer;
mod gameplay_loop;
mod snack_factory;

const WIDTH  : usize = 50;
const HEIGHT : usize = 20;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {detect_user_input(tx)});
    let snake: Snake = snake::new((30, 1), 3);
    let mut dr = display_renderer::new(WIDTH, HEIGHT);
    let sf = snack_factory::new(WIDTH, HEIGHT);
    print!("{esc}c", esc = 27 as char);
    gameplay_loop::play(snake, dr, rx, sf);
}

fn detect_user_input(tx: Sender<char>) -> ! {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let mut reverse = input.chars().rev();
        reverse.next();
        if let Some(last_input) = reverse.next() {
            tx.send(last_input).unwrap();
        }
    }
}
