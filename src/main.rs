use crate::snake::Snake;

mod snake;
mod display_renderer;



fn main() {
    println!("Hello, world!");
    let snake: Snake = snake::new((20, 1));
    println!("{:#?}", snake);
    let mut dr = display_renderer::new(200, 50);
    dr.print_frame(&snake);
}
