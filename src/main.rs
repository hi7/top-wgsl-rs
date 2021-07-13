mod tri;

fn main() {
    let laser = Laser::new();
    tri::init(&laser);
}

pub struct Laser {
    pub widget: tri::Widget,
    energy: f32,
}

impl Laser {
    pub fn new() -> Laser {
        Laser {
            widget: tri::Widget {
                location: (0.0, 0.5)
            },
            energy: 0.5,
        }
    }
}

impl tri::Renders for Laser {
    fn render(&self, vert: &mut [tri::Vertex], idx: &mut [u16], offset: usize) -> usize {
        let indices: [usize; 3] = [2, 5, 6];
        for n in 0..3 {
            let i = n + offset;
            idx[i] = n as u16;
            vert[i].position[0] = RAW_VERTICES[indices[n]].position[0] + self.widget.location.0;
            vert[i].position[1] = RAW_VERTICES[indices[n]].position[1] + self.widget.location.1;
            vert[i].color[0] = 0.02;
            vert[i].color[1] = 0.0;
            vert[i].color[2] = 0.0;
        }
        offset + 3
    }
}

// 0 1 2 3 4
// 5       6
pub(crate) const RAW_VERTICES: &[tri::Vertex] = &[
    tri::Vertex {
        position: [-0.1, 0.1],
        color: [0.01, 0.0, 0.0],
    }, // 0
    tri::Vertex {
        position: [-0.05, 0.1],
        color: [0.01, 0.0, 0.0],
    }, // 1
    tri::Vertex {
        position: [0.0, 0.1],
        color: [0.01, 0.0, 0.0],
    }, // 2
    tri::Vertex {
        position: [0.05, 0.1],
        color: [0.01, 0.0, 0.0],
    }, // 3
    tri::Vertex {
        position: [0.1, 0.1],
        color: [0.01, 0.0, 0.0],
    }, // 4
    tri::Vertex {
        position: [-0.1, -0.1],
        color: [0.01, 0.0, 0.0],
    }, // 5
    tri::Vertex {
        position: [0.1, -0.1],
        color: [0.01, 0.0, 0.0],
    }, // 6
];
//pub(crate) const TRIANGLE_INDICES: &[u16] = &[2, 5, 6];//, 1, 2, 4, 2, 3, 4, /* padding */ 0];
