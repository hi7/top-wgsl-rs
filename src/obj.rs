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

impl Entity for Laser {
    fn update(&mut self) {
        if self.energy < 1.0 {
            self.energy += 0.0001;
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
            vert[i].color[RED] = 0.05;
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
    pub cargo: u8,
}

impl Container {
    pub fn new(x: f32, y: f32) -> Container {
        Container {
            widget: Widget {
                location: (x, y)
            },
            cargo: 1,
        }
    }
}

impl Entity for Container {
    fn update(&mut self) {}
    fn render(&self, vert: &mut [Vertex], idx: &mut [u16], offset: u32) -> u32 {
        let indices: [usize; 6] = [0, 5, 6, 6, 4, 0];
        for n in 0..6 { // back ground
            let i = n + offset as usize;
            idx[i] = i as u16;
            vert[i].position[X] = RAW_VERTICES[indices[n]].position[X] + self.widget.location.0;
            vert[i].position[Y] = RAW_VERTICES[indices[n]].position[Y] + self.widget.location.1;
            vert[i].color[RED] = 0.01;
            vert[i].color[GREEN] = 0.01;
            vert[i].color[BLUE] = 0.01;
        }
        for n in 0..6 { // energy area
            let i = n + 6 + offset as usize;
            idx[i] = i as u16;
            vert[i].position[X] = RAW_VERTICES[indices[n]].position[X] * (self.cargo as f32 / 3.0) + self.widget.location.0;
            vert[i].position[Y] = RAW_VERTICES[indices[n]].position[Y] * (self.cargo as f32 / 3.0) + self.widget.location.1;
            vert[i].color[RED] = 0.1;
            vert[i].color[GREEN] = 0.1;
            vert[i].color[BLUE] = 0.1;
        }
        12
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
