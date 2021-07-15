mod tri;
mod obj;

use crate::obj::{Laser};
use crate::tri::{Entity, Widget};

static mut LASER: Laser = Laser {
    widget: Widget {
        location: (0.0, 0.5)
    },
    energy: 0.3,
};

fn main() {
    unsafe {
        tri::init(&mut LASER);
    }
}

