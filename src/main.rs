mod tri;
mod models;

fn main() {
    let laser = models::Laser::new();
    tri::init(&laser.widget);
}
