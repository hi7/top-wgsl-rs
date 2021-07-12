const NUM_OF_COORDINATES: usize = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    position: [f32; NUM_OF_COORDINATES],
    color: [f32; 3],
}

impl Vertex {
    fn new() -> Vertex {
        Vertex {
            position: [0.0, 0.0],
            color: [0.5, 0.0, 0.0],
        }
    }
}

#[derive(Debug)]
pub struct Widget {
    vertices: [Vertex; 3],
    pub indices: [u16; 3],
    location: (f32, f32),
}

trait Renders {
    fn render(&self);
}

pub struct Laser {
    pub widget: Widget,
    energy: f32,
}

impl Laser {
    pub fn new() -> Laser {
        Laser {
            widget: Widget {
                vertices: [Vertex::new(); 3],
                indices: [2, 5, 6],
                location: (0.05, 0.05)
            },
            energy: 0.5,
        }
    }
}

impl Renders for Laser {
    fn render(&self) {

    }
}

// 0 1 2 3 4
// 5       6
pub(crate) const VERTICES: &[Vertex] = &[
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
pub(crate) const TRIANGLE_INDICES: &[u16] = &[2, 5, 6];//, 1, 2, 4, 2, 3, 4, /* padding */ 0];

impl Vertex {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; NUM_OF_COORDINATES]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
