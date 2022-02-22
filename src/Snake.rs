#[derive(Debug)]
struct SnakeNode {
    pos: (isize, isize),
    sprite: char,
}

#[derive(Debug)]
struct SnakeHead {
    node: SnakeNode,
    prev: Box<dyn Dragable>,
}

#[derive(Debug)]
struct SnakeBody {
    node: SnakeNode,
    prev: Box<dyn Dragable>,
}

#[derive(Debug)]
struct SnakeTail {
    node: SnakeNode
}
#[derive(Debug)]
pub struct Snake {
    head: SnakeHead
}

pub fn new(head_pos: (isize, isize)) -> Snake {
    let tail = make_tail((head_pos.0-2, head_pos.1));
    let body = make_body((head_pos.0-1, head_pos.1), tail);
    let head = make_head(head_pos, body);
    Snake {head: head}
}

fn make_tail(pos: (isize, isize)) -> Box<SnakeTail> {
    let node = SnakeNode {
        pos: pos,
        sprite: 'x' 
    };
    Box::new(SnakeTail {
        node: node
    })
}

fn make_body(pos: (isize, isize), prev: Box<dyn Dragable>) -> Box<SnakeBody> {
    let node = SnakeNode {
        pos: pos,
        sprite: 'x'
    };
    Box::new(SnakeBody {
        node: node,
        prev: prev,
    })
}

fn make_head(pos: (isize, isize), prev: Box<dyn Dragable>) -> SnakeHead {
    let node = SnakeNode {
        pos: pos,
        sprite: '>'
    };
    SnakeHead {
        node: node,
        prev: prev
    }
}

pub trait Movable {
    fn mv(&mut self, displacement: (isize, isize));
}

impl Movable for Snake {
    fn mv(&mut self, displacement: (isize, isize)) {
        self.head.mv(displacement)
    }
}

impl Movable for SnakeHead {
    fn mv(&mut self, displacement: (isize, isize)) {
        let prev_pos = self.node.pos;
        self.node.pos = tuple_sum(self.node.pos, displacement);
        self.adapt_shape(displacement);
        self.prev.drag(prev_pos);
    }
}

trait Dragable: std::fmt::Debug  {
    fn drag(&mut self, target: (isize, isize));
}

impl Dragable for SnakeBody {
    fn drag(&mut self, target: (isize, isize)) {
        let _displacement = tuple_diff(target, self.node.pos);
        let prev_pos = self.node.pos;
        self.node.pos = target;
        self.prev.drag(prev_pos);
    }
}

impl Dragable for SnakeTail {
    fn drag(&mut self, target: (isize, isize)) {
        let _displacement = tuple_diff(target, self.node.pos);
        self.node.pos = target;
    }
}

trait Polymorphic {
    fn adapt_shape(&mut self, displacement: (isize, isize));
}

impl Polymorphic for SnakeHead {
    fn adapt_shape(&mut self, displacement: (isize, isize)) {
        let updated_sprite = match displacement {
            (1,0) => '>',
            (-1,0) => '<',
            (0,1) => 'V',
            (0,-1) => 'A',
            _ => panic!("Impossible displacement. Snake does not move in diagonal."),
        };
        self.node.sprite = updated_sprite;
    }
}

fn tuple_sum(t1: (isize, isize), t2: (isize, isize)) -> (isize, isize) {
    (t1.0 + t2.0, t1.1 + t2.1)
}

fn tuple_diff(t1: (isize, isize), t2: (isize, isize)) -> (isize, isize) {
    (t1.0 - t2.0, t1.1 - t2.1)
}
