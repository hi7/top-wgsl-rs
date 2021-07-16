mod tri;
mod obj;

use crate::obj::{Laser, Container};
use crate::tri::{Entity};

fn main() {
    let entities: Box<[Box<dyn Entity>]> = Box::new([
        Box::new(Laser::new(0.0, 0.2)),
        Box::new(Container::new(0.0, 0.0)),
    ]);
    tri::init(entities);
}

