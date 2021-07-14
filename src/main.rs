mod tri;
mod obj;

fn main() {
    let laser = obj::Laser::new();
    tri::init(&laser);
}

