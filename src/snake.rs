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

pub fn new(head_pos: (isize, isize)) -> Snake {
    let tail = make_tail((head_pos.0-2, head_pos.1));
    let torso = make_torso((head_pos.0-1, head_pos.1), tail);
    let head = make_head(head_pos, torso);
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

trait Body: Dragable + Printable {}

impl Movable for Snake {
    fn mv(&mut self, displacement: (isize, isize)) {
        println!("{:?}", displacement);
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

impl Snake {
    pub fn get_nodes(&self) -> Vec<&SnakeNode> {
        let mut ret : Vec<&SnakeNode> = Vec::new();
        self.head.collect_node(&mut ret);
        ret
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

impl Body for SnakeTorso {}
impl Body for SnakeTail {}