use rand::{thread_rng,Rng};

const SPRITE : char = '*';

pub struct Snack_Factory {
    width  : usize,
    height : usize
}

pub struct Snack {
    pos : (isize, isize)
}

pub(crate) fn new (width : usize, height : usize) -> Snack_Factory {
    Snack_Factory{width,height}
}

impl Snack_Factory {

    pub(crate) fn make_snack(&self, used : &Vec<&(isize, isize)>) -> Snack {
        let mut rng = thread_rng();
        let mut pos = (rng.gen_range(1.. self.width) as isize,
                       rng.gen_range(1.. self.height) as isize);
        while used.iter().any(|&x| pos == *x) {
            pos = (rng.gen_range(1.. self.width) as isize, rng.gen_range(1.. self.height) as isize);
        }
        Snack{pos}
    }
}

impl Snack {

    pub(crate) fn get_pos(&self) -> &(isize, isize) {
        &self.pos
    }

    pub(crate) fn get_sprite(&self) -> char {
        SPRITE
    }
}