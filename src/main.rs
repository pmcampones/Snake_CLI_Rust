mod snake;
mod displayRenderer;

fn main() {
    println!("Hello, world!");
    let snake = snake::new((3,1));
    println!("{:#?}", snake);
    let dr = displayRenderer::new(200, 50);
    dr.print_frame();
}
