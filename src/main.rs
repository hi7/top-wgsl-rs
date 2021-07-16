mod tri;
mod obj;

use crate::obj::{Laser};
use crate::tri::{Widget};

static mut LASER: Laser = Laser {
    widget: Widget {
        location: (0.0, 0.5)
    },
    energy: 0.1,
};

fn main() {
    unsafe {
        tri::init(&mut LASER);
    }
}

