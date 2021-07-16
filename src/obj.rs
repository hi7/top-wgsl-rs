use crate::tri::{Widget, Entity, Vertex, X, Y, RED, GREEN, BLUE};

pub struct Laser {
    pub widget: Widget,
    pub energy: f32,
}

impl Laser {
    pub fn new(x: f32, y: f32) -> Laser {
        Laser {
            widget: Widget {
                location: (x, y)
            },
            energy: 0.0,
        }
    }
}
const RECHARGE: f32 = 0.001;
impl Entity for Laser {
    fn update(&mut self) {
        if self.energy < 1.0 {
            self.energy += RECHARGE;
            if self.energy > 1.0 { self.energy = 1.0 }
        }
    }
    fn render(&self, vert: &mut [Vertex], idx: &mut [u16], offset: u32) -> u32 {
        let indices: [usize; 3] = [2, 5, 6];
        for n in 0..3 { // back ground
            let i = n + offset as usize;
            idx[i] = i as u16;
            vert[i].position[X] = RAW_VERTICES[indices[n]].position[X] + self.widget.location.0;
            vert[i].position[Y] = RAW_VERTICES[indices[n]].position[Y] + self.widget.location.1;
            vert[i].color[RED] = 0.02;
            vert[i].color[GREEN] = 0.0;
            vert[i].color[BLUE] = 0.0;
        }
        for n in 0..3 { // energy area
            let i = n + 3 + offset as usize;
            idx[i] = i as u16;
            if n == 0 {
                vert[i].position[X] = RAW_VERTICES[indices[n]].position[X] + self.widget.location.0;
                vert[i].position[Y] = RAW_VERTICES[indices[n]].position[Y] + self.widget.location.1;
            } else {
                vert[i].position[X] =
                    RAW_VERTICES[indices[n]].position[X] * self.energy
                    + self.widget.location.0;
                vert[i].position[Y] =
                    (RAW_VERTICES[indices[n]].position[Y] - 0.1) * self.energy + 0.1
                        + self.widget.location.1;
            }
            vert[i].color[RED] = 0.5;
            vert[i].color[GREEN] = 0.0;
            vert[i].color[BLUE] = 0.0;
        }
        6
    }
}

pub struct Container {
    pub widget: Widget,
    pub capacity: u8,
    pub cargo: u8,
    pub laser_energy: f32,
    pub thrust_energy: f32,
}

impl Container {
    pub fn new(x: f32, y: f32, cargo: u8) -> Container {
        Container {
            widget: Widget {
                location: (x, y)
            },
            capacity: 10,
            cargo,
            laser_energy: 0.0,
            thrust_energy: 0.0,
        }
    }
}

impl Entity for Container {
    fn update(&mut self) {
    }
    fn render(&self, vert: &mut [Vertex], idx: &mut [u16], offset: u32) -> u32 {
        let indices: [usize; 6] = [0, 5, 6, 6, 4, 0];
        for n in 0..6 { // back ground
            let i = n + offset as usize;
            idx[i] = i as u16;
            vert[i].position[X] = RAW_VERTICES[indices[n]].position[X] + self.widget.location.0;
            vert[i].position[Y] = RAW_VERTICES[indices[n]].position[Y] + self.widget.location.1;
            vert[i].color[RED] = 0.02;
            vert[i].color[GREEN] = 0.0;
            vert[i].color[BLUE] = 0.0;
        }
        for n in 0..6 { // thrust energy area
            let i = n + 6 + offset as usize;
            idx[i] = i as u16;
            if indices[n] == 0 || indices[n] == 4 {
                vert[i].position[X] = RAW_VERTICES[indices[n]].position[X] + self.widget.location.0;
                vert[i].position[Y] = RAW_VERTICES[indices[n]].position[Y] + self.widget.location.1;
            } else {
                vert[i].position[X] = RAW_VERTICES[indices[n]].position[X] + self.widget.location.0;
                vert[i].position[Y] = RAW_VERTICES[indices[n]].position[Y] * (self.laser_energy)
                    + 0.1 + self.widget.location.1;
            }
            vert[i].color[RED] = 0.5;
            vert[i].color[GREEN] = 0.0;
            vert[i].color[BLUE] = 0.0;
        }
        for n in 0..6 { // cargo area
            let i = n + 12 + offset as usize;
            idx[i] = i as u16;
            vert[i].position[X] = RAW_VERTICES[indices[n]].position[X] + self.widget.location.0;
            vert[i].position[Y] = RAW_VERTICES[indices[n]].position[Y]
                * (self.cargo as f32 / self.capacity as f32) + self.widget.location.1;
            vert[i].color[RED] = 0.1;
            vert[i].color[GREEN] = 0.1;
            vert[i].color[BLUE] = 0.1;
        }
        18
    }
}

pub struct Ship {
    pub laser: Laser,
    pub container: Container,
}

impl Ship {
    pub fn new(x: f32, y: f32, cargo: u8) -> Ship {
        Ship {
            laser: Laser::new(x, y + 0.2),
            container: Container::new(x, y, cargo),
        }
    }
}

impl Entity for Ship {
    fn update(&mut self) {
        self.laser.update();
        if self.laser.energy == 1.0 {
            self.container.laser_energy += RECHARGE;
            if self.container.cargo == 0 {
                if self.container.laser_energy > 2.0 {
                    self.container.laser_energy = 2.0;
                }
            } else if self.container.laser_energy > 1.0
                      - self.container.cargo as f32/ self.container.capacity as f32 {
                self.container.laser_energy = 1.0
                    - self.container.cargo as f32 / self.container.capacity as f32;
            }
        }
        self.container.update();
    }
    fn render(&self, vert: &mut [Vertex], idx: &mut [u16], offset: u32) -> u32 {
        let mut count = self.laser.render(vert, idx, offset);
        count += self.container.render(vert, idx, offset + count);
        offset + count
    }
}

// 0 1 2 3 4
// 5       6
pub(crate) const RAW_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.1, 0.1],
        color: [0.01, 0.0, 0.0],
    }, // 0
    Vertex {
        position: [-0.05, 0.1],
        color: [0.01, 0.0, 0.0],
    }, // 1
    Vertex {
        position: [0.0, 0.1],
        color: [0.01, 0.0, 0.0],
    }, // 2
    Vertex {
        position: [0.05, 0.1],
        color: [0.01, 0.0, 0.0],
    }, // 3
    Vertex {
        position: [0.1, 0.1],
        color: [0.01, 0.0, 0.0],
    }, // 4
    Vertex {
        position: [-0.1, -0.1],
        color: [0.01, 0.0, 0.0],
    }, // 5
    Vertex {
        position: [0.1, -0.1],
        color: [0.01, 0.0, 0.0],
    }, // 6
];
