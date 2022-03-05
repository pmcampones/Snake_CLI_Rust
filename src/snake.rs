use std::mem;
use std::ptr::null;
use crate::snack_factory::Snack;

const DEFAULT_BODY : char = '-';
const DEFAULT_HEAD : char = '>';

const H_BODY : char = '-';
const V_BODY : char = '|';

const LFT_HEAD : char = '<';
const RGT_HEAD : char = '>';
const UP_HEAD  : char = 'A';
const DWN_HEAD : char = 'V';

const LFT_MV : (isize, isize) = (-1, 0);
const RGT_MV : (isize, isize) = (1, 0);
const UP_MV  : (isize, isize) = (0, -1);
const DWN_MV : (isize, isize) = (0, 1);

#[derive(Debug)]
pub struct SnakeNode {
    pub pos: (isize, isize),
    pub sprite: char,
}

#[derive(Debug)]
struct SnakeBody {
    node: SnakeNode,
    prev: Option<Box<SnakeBody>>,
    is_digesting: bool
}

#[derive(Debug)]
struct SnakeHead {
    node: SnakeNode,
    prev: Option<Box<SnakeBody>>,
    is_digesting: bool
}

#[derive(Debug)]
pub struct Snake {
    head: SnakeHead
}

pub fn new(head_pos: (isize, isize), size: isize) -> Snake {
    if size < 1 {
        panic!("Snake size must be at least 1 (head)");
    }
    let mut prev : Option<Box<SnakeBody>> = None;
    for i in (1 .. size).rev() {
        prev = Some(make_body_part((head_pos.0 - i, head_pos.1), prev));
    }
    let head = make_head(head_pos, prev);
    Snake{head}
}

fn make_body_part(pos: (isize, isize), prev: Option<Box<SnakeBody>>) -> Box<SnakeBody> {
    let node = SnakeNode {pos, sprite: DEFAULT_BODY};
    Box::new(SnakeBody {node, prev, is_digesting: false})
}

fn make_head(pos: (isize, isize), prev: Option<Box<SnakeBody>>) -> SnakeHead {
    let node = SnakeNode {pos, sprite: DEFAULT_HEAD};
    SnakeHead {node, prev, is_digesting: false}
}

impl Snake {

    pub fn mv(&mut self, displacement: (isize, isize)) {
        self.head.mv(displacement)
    }

    pub fn get_nodes(&self) -> Vec<&SnakeNode> {
        let mut ret : Vec<&SnakeNode> = Vec::new();
        self.head.collect_node(&mut ret);
        ret
    }

    pub fn get_pos(&self) -> Vec<&(isize,isize)> {
        self.get_nodes().iter().map(|&x| &(x.pos)).collect::<Vec<_>>()
    }

    pub fn get_head_pos(&self) -> (isize, isize) {
        self.head.node.pos
    }

    pub fn is_eating_self(&self) -> bool {
        let node_pos = self.get_nodes();
        let head_pos = node_pos[0].pos;
        let body_pos = &node_pos[1..];
        body_pos.iter().any(|&x| head_pos == x.pos)
    }

    pub fn can_eat(&self, snack: &Snack) -> bool {
        self.head.node.pos == *snack.get_pos()
    }

    pub fn eat_snack(&mut self) {
        self.head.eat_snack();
    }
}

impl SnakeHead {

    fn mv(&mut self, displacement: (isize, isize)) {
        match &mut self.prev {
            Some(p) => {
                p.drag(self.node.pos);
                if self.is_digesting {
                    p.eat_snack();
                }
            },
            None => {
                if self.is_digesting {
                    let node = SnakeNode {pos : self.node.pos, sprite: self.node.sprite};
                    let tail = Box::new(SnakeBody {node, prev: None, is_digesting: false});
                    self.prev = Some(tail);
                }
            }
        }
        self.adapt_shape(displacement);
        self.node.pos = tuple_sum(self.node.pos, displacement);
        self.is_digesting = false;
    }

    fn eat_snack(&mut self) {
        self.is_digesting = true;
    }

    fn adapt_shape(&mut self, displacement: (isize, isize)) {
        let updated_sprite = match displacement {
            LFT_MV => LFT_HEAD,
            RGT_MV => RGT_HEAD,
            UP_MV => UP_HEAD,
            DWN_MV => DWN_HEAD,
            _ => panic!("Impossible displacement. Snake does not move in diagonal."),
        };
        self.node.sprite = updated_sprite;
    }

    fn collect_node<'a>(& 'a self, vec: & mut Vec<& 'a SnakeNode>) {
        vec.push(&self.node);
        if let Some(p) = &self.prev {
            p.collect_node(vec);
        }
    }
}

impl SnakeBody {

    fn update_sprite(& mut self, update: char) {
        self.node.sprite = update;
    }

    fn eat_snack(&mut self) {
        self.is_digesting = true;
    }

    fn drag(&mut self, target: (isize, isize)) {
        match &mut self.prev {
            Some(p) => {
                p.drag(self.node.pos);
                if self.is_digesting {
                    p.eat_snack();
                }
            },
            None => {
                if self.is_digesting {
                    let node = SnakeNode {pos : self.node.pos, sprite: self.node.sprite};
                    let tail = Box::new(SnakeBody {node, prev: None, is_digesting: false});
                    self.prev = Some(tail);
                }
            }
        }
        let displacement = tuple_diff(target, self.node.pos);
        self.adapt_shape(displacement);
        self.node.pos = target;
        self.is_digesting = false;
    }

    fn collect_node<'a>(&'a self, vec: & mut Vec<& 'a SnakeNode>) {
        vec.push(&self.node);
        if let Some(p) = &self.prev {
            p.collect_node(vec);
        }
    }

    fn get_pos(&self) -> &(isize, isize) {
        &self.node.pos
    }

    fn get_sprite(&self) -> char {
        self.node.sprite
    }

    fn adapt_shape(&mut self, displacement: (isize, isize)) {
        if displacement.0 != 0 {
            self.update_sprite(H_BODY);
        } else if displacement.1 != 0 {
            self.update_sprite(V_BODY);
        }
    }

}

fn tuple_sum(t1: (isize, isize), t2: (isize, isize)) -> (isize, isize) {
    (t1.0 + t2.0, t1.1 + t2.1)
}

fn tuple_diff(t1: (isize, isize), t2: (isize, isize)) -> (isize, isize) {
    (t1.0 - t2.0, t1.1 - t2.1)
}
