

pub struct DisplayRenderer {
    width: isize,
    height: isize,
}

pub fn new(width: isize, height: isize) -> DisplayRenderer {
    DisplayRenderer {width: width, height: height}
}

impl DisplayRenderer {
    pub fn print_frame(&self) {
        let mut top_bot = String::new();
        for _ in 0 .. self.width {
            top_bot.push('#');
        }

        let mut mid = String::new();
        mid.push('#');
        for _ in 1 .. (self.width - 1) {
            mid.push(' ');
        }
        mid.push('#');

        println!("{}",top_bot);
        for _ in 1 .. (self.height - 1) {
            println!("{}", mid);
        }
        println!("{}", top_bot);
    }
}
