mod Snake;

fn main() {
    println!("Hello, world!");
    let snake = Snake::new((3,1));
    println!("{:#?}", snake)
}
