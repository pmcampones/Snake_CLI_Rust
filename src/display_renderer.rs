use crate::snack_factory::Snack;
use crate::snake::Snake;

pub struct DisplayRenderer {
    width: usize,
    height: usize,
    frame: Vec<char>
}

pub fn new(width: usize, height: usize) -> DisplayRenderer {
    let frame = initialize_frame_matrix(width, height);
    DisplayRenderer {width, height, frame}
}

fn initialize_frame_matrix(width: usize, height: usize) -> Vec<char> {
    let mut frame = Vec::with_capacity((width * height) as usize);
    for _ in 0..(width * height) {
        frame.push(' ');
    }
    frame
}

impl DisplayRenderer {
    pub fn next_frame(&mut self, snake : &Snake, snack : &Snack) {
        self.clear_frame();
        self.write_walls();
        self.write_snake(snake);
        self.write_snack(snack);
        self.print_frame();
    }

    fn write_snack(&mut self, snack: &Snack) {
        let pos = (snack.get_pos().0 as usize, snack.get_pos().1 as usize);
        self.frame[pos.1 * self.width + pos.0] = snack.get_sprite();
    }

    fn print_frame(&self) {
        for i in 0..self.height {
            let mut line = String::with_capacity(self.width);
            for j in 0..self.width {
                line.push(self.frame[i * self.width + j]);
            }
            println!("{}", line);
        }
    }

    fn clear_frame(& mut self) {
        for i in 0 .. (self.width * self.height) {
            self.frame[i] = ' ';
        }
    }

    fn write_walls(& mut self) {
        for i in 0 .. self.width {
            self.frame[i] = '#';
        }
        for i in 1 .. (self.height - 1) {
            self.frame[i * self.width] = '#';
        }
        for i in 2 .. self.height {
            self.frame[i * self.width - 1] = '#';
        }
        for i in (self.frame.len() - self.width) .. self.frame.len() {
            self.frame[i] = '#';
        }
    }

    fn write_snake(&mut self, snake : &Snake) {
        let snake_nodes = snake.get_nodes();
        for node in snake_nodes {
            let pos = (node.pos.0 as usize, node.pos.1 as usize);
            self.frame[pos.1 * self.width + pos.0] = node.sprite;
        }
    }

    fn gen_wall_line(&self) -> String {
        let mut top_bot = String::new();
        for _ in 0 .. self.width {
            top_bot.push('#');
        }
        top_bot
    }

    pub fn is_in_wall(&self, pos: (isize, isize)) -> bool {
        pos.0 > 0
        && pos.0 < (self.width - 1) as isize
        && pos.1 > 0
        && pos.1 < (self.height - 1) as isize
    }
}