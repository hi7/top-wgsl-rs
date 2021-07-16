mod tri;
mod obj;

use crate::obj::{Ship};
use crate::tri::{Entity};

fn main() {
    let entities: Box<[Box<dyn Entity>]> = Box::new([
        Box::new(Ship::new(0.0, 0.0)),
    ]);
    tri::init(entities);
}

