use crate::tri::{Widget, Renders, Vertex};

pub struct Laser {
    pub widget: Widget,
    energy: f32,
}

impl Laser {
    pub fn new() -> Laser {
        Laser {
            widget: Widget {
                location: (0.0, 0.5)
            },
            energy: 0.5,
        }
    }
}

impl Renders for Laser {
    fn render(&self, vert: &mut [Vertex], idx: &mut [u16], offset: u32) -> u32 {
        let indices: [usize; 3] = [2, 5, 6];
        for n in 0..3 {
            let i = n + offset as usize;
            idx[i] = n as u16;
            vert[i].position[0] = RAW_VERTICES[indices[n]].position[0] + self.widget.location.0;
            vert[i].position[1] = RAW_VERTICES[indices[n]].position[1] + self.widget.location.1;
            vert[i].color[0] = 0.02;
            vert[i].color[1] = 0.0;
            vert[i].color[2] = 0.0;
        }
        for n in 0..3 {
            let i = n + 3 + offset as usize;
            idx[i] = (n + 3) as u16;
            vert[i].position[0] = RAW_VERTICES[indices[n]].position[0] * self.energy + self.widget.location.0;
            vert[i].position[1] = RAW_VERTICES[indices[n]].position[1] * self.energy + self.widget.location.1;
            vert[i].color[0] = 0.5;
            vert[i].color[1] = 0.0;
            vert[i].color[2] = 0.0;
        }
        offset + 6
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
