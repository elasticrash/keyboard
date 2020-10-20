pub fn transform(n: [f32; 2], horizontal_offset: f32, vertical_offset: f32) -> [f32; 2] {
    [
        (n[0] + horizontal_offset) / 3.,
        (n[1] + vertical_offset) / 3.,
    ]
}

pub fn transform_single(n: f32) -> f32 {
    n / 3.
}
