use crate::config::model::Layout;
use crate::transformations::transform;
use crate::transformations::transform_single;
use dxf::entities::Polyline;
use egaku2d::sprite::SpriteSession;
use egaku2d::SimpleCanvas;

pub fn generate_virtual_keyboard(
    canvas: &mut SimpleCanvas,
    mut sprites: &mut SpriteSession,
    keyboard: &Layout,
    geometry: &Vec<Polyline>,
) {
    let mut lines = canvas.lines(1.3);
    let mut rects = canvas.rects();
    let mut rects_inner = canvas.rects();

    let mut row = 1;
    let mut pos_y = 150.;
    let u1_size = 190.;
    let gap = 0.5;
    let ff = 0.1;
    let mut base_width = 0.0;

    for x in &keyboard.layout {
        let mut point = 1.0;
        let mut pos_x = 50.;
        for y in x {
            if y.k_type == 1 {
                rects.add(rectangle(pos_x, pos_y, u1_size, row, point, y.size, 0.0));
                rects_inner.add(rectangle(pos_x, pos_y, u1_size, row, point, y.size, 1.));

                add_ascii(
                    [
                        transform_single(pos_x + (point * u1_size) + u1_size / 3. as f32),
                        transform_single(pos_y + ((row as f32) * u1_size) + u1_size / 3. as f32),
                    ],
                    10.0,
                    0.0,
                    &y.display,
                    &mut sprites,
                );
            }
            point += y.size;
            pos_x += gap;

            if row == 1 {
                base_width = base_width + u1_size * y.size;
            }
        }
        pos_y += 0.5;
        row += 1;
        rects
            .send_and_uniforms(canvas)
            .with_color([ff * row as f32, 0.0, 0.0, 1.0])
            .draw();
        rects_inner
            .send_and_uniforms(canvas)
            .with_color([ff, 1.0, 0.4, 1.0])
            .draw();
        rects_inner = canvas.rects();
        rects = canvas.rects();
    }
    let v_offset = 2000.;
    let h_offset = 200.;

    for pl in geometry {
        for vtx in 0..pl.vertices.len() {
            if vtx + 1 < pl.vertices.len() {
                lines.add(
                    transform(
                        [{ pl.vertices[vtx].location.x as f32 }, {
                            -pl.vertices[vtx].location.y as f32
                        }],
                        h_offset,
                        v_offset,
                    ),
                    transform(
                        [{ pl.vertices[vtx + 1].location.x as f32 }, {
                            -pl.vertices[vtx + 1].location.y as f32
                        }],
                        h_offset,
                        v_offset,
                    ),
                );
            }
        }
    }
    lines
        .send_and_uniforms(canvas)
        .with_color([ff * row as f32, 0.6, 0.4, 1.0])
        .draw();
    canvas.lines(1.3);
}

fn rectangle(x: f32, y: f32, s: f32, r: i32, p: f32, key: f32, border: f32) -> [f32; 4] {
    return [
        transform_single(x + (p * s) as f32) + border,
        transform_single(x + (p * s + (key * s)) as f32) - border,
        transform_single(y + ((r as f32) * s) as f32) + border,
        transform_single(y + ((r as f32) * s + s) as f32) - border,
    ];
}

pub fn add_ascii(
    start: [f32; 2],
    width: f32,
    rotation: f32,
    st: &str,
    sprites: &mut egaku2d::sprite::SpriteSession,
) {
    let mut cc = start;
    for (_i, a) in st.chars().enumerate() {
        let ascii = a as u8;
        assert!(ascii >= 32);
        sprites.add(cc, (ascii - 32) as u16, rotation);
        cc[0] += width;
    }
}
