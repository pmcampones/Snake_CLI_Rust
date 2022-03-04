use crate::snake::Snake;

use std::{io, thread};
use std::sync::mpsc;

mod snake;
mod display_renderer;
mod gameplay_loop;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            io::stdin().read_line(& mut input);
            let mut reverse = input.chars().rev();
            reverse.next();
            let last_input = reverse.next().unwrap();
            //println!("Last input is {}", last_input);
            tx.send(last_input).unwrap();
        }
    });

    println!("Hello, world!");
    let snake: Snake = snake::new((20, 1));
    println!("{:#?}", snake);
    let mut dr = display_renderer::new(100, 20);
    gameplay_loop::play(snake, dr, rx);
}
