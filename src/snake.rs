const DEFAULT_BODY : char = '-';
const DEFAULT_HEAD : char = '>';

const H_BODY : char = '-';
const V_BODY : char = '|';

const UP_HEAD  : char = 'A';
const DWN_HEAD : char = 'V';
const LFT_HEAD : char = '<';
const RGT_HEAD : char = '>';

#[derive(Debug)]
pub struct SnakeNode {
    pub pos: (isize, isize),
    pub sprite: char,
}

#[derive(Debug)]
struct SnakeHead {
    node: SnakeNode,
    prev: Box<dyn Body>
}

#[derive(Debug)]
struct SnakeTorso {
    node: SnakeNode,
    prev: Box<dyn Body>
}

#[derive(Debug)]
struct SnakeTail {
    node: SnakeNode
}
#[derive(Debug)]
pub struct Snake {
    head: SnakeHead
}

pub fn new(head_pos: (isize, isize), size: isize) -> Snake {
    if size < 2 {
        panic!("Snake size must be at least 2 (head and tail)");
    }
    let tail = make_tail((head_pos.0-(size - 1), head_pos.1));
    let mut prev_body = tail as Box<dyn Body>;
    for i in (1 .. (size - 1)).rev() {
        prev_body = make_torso((head_pos.0 - i, head_pos.1), prev_body);
    }
    let head = make_head(head_pos, prev_body);
    Snake {head}
}

fn make_tail(pos: (isize, isize)) -> Box<SnakeTail> {
    let node = SnakeNode {pos, sprite: DEFAULT_BODY};
    Box::new(SnakeTail {node})
}

fn make_torso(pos: (isize, isize), prev: Box<dyn Body>) -> Box<SnakeTorso> {
    let node = SnakeNode {pos, sprite: DEFAULT_BODY};
    Box::new(SnakeTorso {node, prev})
}

fn make_head(pos: (isize, isize), prev: Box<dyn Body>) -> SnakeHead {
    let node = SnakeNode {pos, sprite: DEFAULT_HEAD};
    SnakeHead {node, prev}
}

pub trait Movable {
    fn mv(&mut self, displacement: (isize, isize));
}

trait Dragable: std::fmt::Debug  {
    fn drag(&mut self, target: (isize, isize));
}

trait Printable {
    fn collect_node<'a>(& 'a self, vec : & mut Vec<& 'a SnakeNode>);
}

trait Body: Dragable + Printable {
    fn update_sprite(& mut self, update: char);
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

impl Dragable for SnakeTorso {
    fn drag(&mut self, target: (isize, isize)) {
        let displacement = tuple_diff(target, self.node.pos);
        let prev_pos = self.node.pos;
        self.node.pos = target;
        (self as &mut dyn Body).adapt_shape(displacement);
        self.prev.drag(prev_pos);
    }
}

impl Dragable for SnakeTail {
    fn drag(&mut self, target: (isize, isize)) {
        let displacement = tuple_diff(target, self.node.pos);
        self.node.pos = target;
        (self as &mut dyn Body).adapt_shape(displacement);
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

impl Polymorphic for dyn Body {
    fn adapt_shape(&mut self, displacement: (isize, isize)) {
        let update = if displacement.0 != 0 {
            H_BODY
        } else if displacement.1 != 0 {
            V_BODY
        } else {
            panic!("Invalid displacement")
        };
        self.update_sprite(update)
    }
}

fn tuple_sum(t1: (isize, isize), t2: (isize, isize)) -> (isize, isize) {
    (t1.0 + t2.0, t1.1 + t2.1)
}

fn tuple_diff(t1: (isize, isize), t2: (isize, isize)) -> (isize, isize) {
    (t1.0 - t2.0, t1.1 - t2.1)
}

impl Snake {
    pub fn get_nodes(&self) -> Vec<&SnakeNode> {
        let mut ret : Vec<&SnakeNode> = Vec::new();
        self.head.collect_node(&mut ret);
        ret
    }

    pub fn get_pos(&self) -> (isize, isize) {
        self.head.node.pos
    }

    pub fn is_eating_self(&self) -> bool {
        let node_pos = self.get_nodes();
        let head_pos = node_pos[0].pos;
        let body_pos = &node_pos[1..];  //TODO Find how to use lambdas ("any match" would be ideal here)
        for pos in body_pos {
            if head_pos == pos.pos {
                return true
            }
        }
        false
    }
}

impl Printable for SnakeHead {
    fn collect_node<'a>(& 'a self, vec: & mut Vec<& 'a SnakeNode>) {
        vec.push(&self.node);
        self.prev.collect_node(vec);
    }
}

impl Printable for SnakeTorso {
    fn collect_node<'a>(&'a self, vec: & mut Vec<& 'a SnakeNode>) {
        vec.push(&self.node);
        self.prev.collect_node(vec);
    }
}

impl Printable for SnakeTail {
    fn collect_node<'a>(& 'a self, vec: &mut Vec<& 'a SnakeNode>) {
        vec.push(&self.node);
    }
}

impl Body for SnakeTorso {
    fn update_sprite(& mut self, update: char) {
        self.node.sprite = update;
    }
}
impl Body for SnakeTail {
    fn update_sprite(&mut self, update: char) {
        self.node.sprite = update;
    }
}