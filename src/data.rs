use crate::Vertex;

pub fn get_data() -> Vec<f32> {
    return vec![0.1];
}

pub fn create_vertices(x: f32, width: f32, height: f32) -> Vec<Vertex> {
    let bottom: f32 = -0.75;
    let top: f32 = bottom + height;
    let right: f32 = x + width;

    return vec![
        Vertex {
            position: [x, bottom, 0.0],
            color: [0.0; 3],
        },
        Vertex {
            position: [right, bottom, 0.0],
            color: [0.0; 3],
        },
        Vertex {
            position: [right, top, 0.0],
            color: [0.0; 3],
        },
        Vertex {
            position: [x, bottom, 0.0],
            color: [0.0; 3],
        },
        Vertex {
            position: [right, top, 0.0],
            color: [0.0; 3],
        },
        Vertex {
            position: [x, top, 0.0],
            color: [0.0; 3],
        },
    ];
}
