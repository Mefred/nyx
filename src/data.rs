pub fn get_data() -> Vec<f32> {
    return vec![0.1];
}

pub fn create_vertices(x: f32, width: f32, height: f32) -> Vec<[f32; 2]> {
    let bottom: f32 = -0.75;
    let top: f32 = bottom + width;
    let right: f32 = x + width;

    return vec![[x, bottom], [right, bottom], [right, top], [x, bottom]];
}
