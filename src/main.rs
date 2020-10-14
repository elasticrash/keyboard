extern crate egaku2d;
extern crate ply_rs;

use crate::exported_geometry::get_geometry;
use dxf::entities::*;
use dxf::Drawing;
use egaku2d::glutin::event::{Event, VirtualKeyCode, WindowEvent};
use egaku2d::glutin::event_loop::ControlFlow;
use ply_rs::ply::{
    Addable, DefaultElement, ElementDef, Encoding, Ply, Property, PropertyDef, PropertyType,
    ScalarType,
};
use ply_rs::writer::Writer;
use rtriangulate::{triangulate, TriangulationPoint};
use std::env;
use std::io::Write;
mod config;
mod exported_geometry;
use dxf::Point;
use std::fs::File;

fn main() {
    let events_loop = egaku2d::glutin::event_loop::EventLoop::new();
    let mut sys = egaku2d::WindowedSystem::new([1200, 1000], &events_loop, "keyboard designer");
    let args: Vec<String> = env::args().collect();
    let file: &str = args[1].as_ref();
    let name: String = file.to_owned();
    let ascii_tex = sys.texture("ascii.png", [16, 14]).unwrap();

    let keyboard = config::reader::read(format!("{}.json", file).as_str()).unwrap();

    //Draw 60 frames per second.
    let mut timer = egaku2d::RefreshTimer::new(16);
    let geometry = get_geometry(&keyboard);
    let mut drawing = Drawing::default();

    let mut tpoints: Vec<TriangulationPoint<f64>> = vec![];
    let mut vetrices: Vec<Point> = vec![];

    for pl in geometry {
        for vtx in &pl.vertices {
            tpoints.push(TriangulationPoint::new(vtx.location.x, vtx.location.y));
            vetrices.push(Point::new(vtx.location.x, vtx.location.y, 0.));
        }

        drawing.entities.push(Entity::new(EntityType::Polyline(pl)));
    }

    events_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => {
                    *control_flow = ControlFlow::Exit;
                }
                Some(VirtualKeyCode::D) => {
                    drawing.save_file(&format!("{}.dxf", name));
                }
                Some(VirtualKeyCode::P) => {
                    let t_triangles = triangulate(&tpoints).unwrap();

                    let mut ply = Ply::<DefaultElement>::new();
                    ply.header.encoding = Encoding::Ascii;
                    ply.header.comments.push("A beautiful comment!".to_string());
                    // Add data
                    let mut points = Vec::new();
                    let mut triangles = Vec::new();
                    let mut point_element = ElementDef::new("vertex".to_string());
                    let p =
                        PropertyDef::new("x".to_string(), PropertyType::Scalar(ScalarType::Float));
                    point_element.properties.add(p);
                    let p =
                        PropertyDef::new("y".to_string(), PropertyType::Scalar(ScalarType::Float));
                    point_element.properties.add(p);
                    let p =
                        PropertyDef::new("z".to_string(), PropertyType::Scalar(ScalarType::Float));
                    point_element.properties.add(p);
                    ply.header.elements.add(point_element);
                    for vtx in &vetrices {
                        let mut point = DefaultElement::new();
                        point.insert("x".to_string(), Property::Float(vtx.x as f32));
                        point.insert("y".to_string(), Property::Float(vtx.y as f32));
                        point.insert("z".to_string(), Property::Float(vtx.z as f32));
                        points.push(point);
                    }
                    ply.payload.insert("vertex".to_string(), points);
                    ply.make_consistent().unwrap();
                    let mut face_element = ElementDef::new("face".to_string());
                    let p = PropertyDef::new(
                        "vertex_index".to_string(),
                        PropertyType::List(ScalarType::UChar, ScalarType::Int),
                    );
                    face_element.properties.add(p);
                    ply.header.elements.add(face_element);
                    for tr in t_triangles {
                        let mut triangle = DefaultElement::new();
                        triangle.insert(
                            "vertex_index".to_string(),
                            Property::ListInt(vec![tr.0 as i32, tr.1 as i32, tr.2 as i32]),
                        );
                        triangles.push(triangle);
                    }
                    ply.payload.insert("face".to_string(), triangles);
                    ply.make_consistent().unwrap();
                    let mut buf = Vec::<u8>::new();
                    let w = Writer::new();
                    let written = w.write_ply(&mut buf, &mut ply).unwrap();
                    let mut file = File::create(format!("{}.ply", name)).unwrap();
                    file.write_all(&buf).unwrap();
                }
                _ => {}
            },
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(_dim) => {}
            _ => {}
        },
        Event::MainEventsCleared => {
            if timer.is_ready() {
                let canvas = sys.canvas_mut();

                canvas.clear_color([0.2; 3]);

                let mut rects = canvas.rects();
                let mut rects_inner = canvas.rects();

                let mut sprites = canvas.sprites();
                let mut lines = canvas.lines(1.3);

                let mut row = 1;
                let mut pos_y = 150.;
                let u1_size = 190.;
                let gap = 0.5;
                let ff = 0.1;
                let mut base_width = 0.0;

                add_ascii([10., 10.], 10.0, 0.0, "D: exports dxf", &mut sprites);
                add_ascii([10., 20.], 10.0, 0.0, "P: exports ply", &mut sprites);

                for x in &keyboard.layout {
                    let mut point = 1.0;
                    let mut pos_x = 50.;
                    for y in x {
                        if y.k_type == 1 {
                            rects.add(rectangle(pos_x, pos_y, u1_size, row, point, y.size, 0.0));
                            rects_inner
                                .add(rectangle(pos_x, pos_y, u1_size, row, point, y.size, 1.));

                            add_ascii(
                                [
                                    transform_single(
                                        pos_x + (point * u1_size) + u1_size / 3. as f32,
                                    ),
                                    transform_single(
                                        pos_y + ((row as f32) * u1_size) + u1_size / 3. as f32,
                                    ),
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

                pos_y = 1400.;
                base_width += 200.;
                let mut pos_x = 150.;

                // frame
                lines.add(
                    transform([0.0 + pos_x, 0.0 + pos_y], 0.),
                    transform([base_width + pos_x, 0.0 + pos_y], 0.),
                );
                lines.add(
                    transform([base_width + pos_x, 0.0 + pos_y], 0.),
                    transform([base_width + pos_x, (row * 190) as f32 + pos_y], 0.),
                );
                lines.add(
                    transform([base_width + pos_x, (row * 190) as f32 + pos_y], 0.),
                    transform([0.0 + pos_x, (row * 190) as f32 + pos_y], 0.),
                );
                lines.add(
                    transform([0.0 + pos_x, (row * 190) as f32 + pos_y], 0.),
                    transform([0.0 + pos_x, 0.0 + pos_y], 0.),
                );
                pos_y = 1500.;

                for x in &keyboard.layout {
                    pos_x = 250.;
                    for y in x {
                        let offset = (((y.size - 1.) * 190.) / 2.) as f32;
                        if y.size < 2. && y.k_type == 1 {
                            let switch = switch_slot(pos_x, pos_y);
                            for swln_index in 0..switch.len() {
                                lines.add(
                                    transform(switch[swln_index][0], offset),
                                    transform(switch[swln_index][1], offset),
                                );
                            }
                        } else if y.size == 6.25 && y.k_type == 1 {
                            let stabilizer = stabilizer(pos_x, pos_y, 381.5);
                            for swln_index in 0..stabilizer.len() {
                                lines.add(
                                    transform(stabilizer[swln_index][0], offset),
                                    transform(stabilizer[swln_index][1], offset),
                                );
                            }
                        } else if y.k_type == 1 {
                            let stabilizer = stabilizer(pos_x, pos_y, 0.0);
                            for swln_index in 0..stabilizer.len() {
                                lines.add(
                                    transform(stabilizer[swln_index][0], offset),
                                    transform(stabilizer[swln_index][1], offset),
                                );
                            }
                        }
                        pos_x += 190.5 + offset * 2.;
                    }
                    pos_y += 190.5;
                }

                lines
                    .send_and_uniforms(canvas)
                    .with_color([ff * row as f32, 0.6, 0.4, 1.0])
                    .draw();
                lines = canvas.lines(1.3);

                sprites.send_and_uniforms(canvas, &ascii_tex, 10.0).draw();

                //display what we drew
                sys.swap_buffers();
            }
        }
        _ => {}
    });
}

fn add_ascii(
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

fn rectangle(x: f32, y: f32, s: f32, r: i32, p: f32, key: f32, border: f32) -> [f32; 4] {
    return [
        transform_single(x + (p * s) as f32) + border,
        transform_single(x + (p * s + (key * s)) as f32) - border,
        transform_single(y + ((r as f32) * s) as f32) + border,
        transform_single(y + ((r as f32) * s + s) as f32) - border,
    ];
}

fn switch_slot(x: f32, y: f32) -> [[[f32; 2]; 2]; 20] {
    return [
        [[x + 8., y], [x + 148., y]],
        [[x + 148., y], [x + 148., y + 10.]],
        [[x + 148., y + 10.], [x + 156., y]],
        [[x + 156., y + 10.], [x + 156., y + 41.]],
        [[x + 156., y + 41.], [x + 148., y + 41.]],
        [[x + 148., y + 41.], [x + 148., y + 99.]],
        [[x + 148., y + 99.], [x + 156., y + 99.]],
        [[x + 156., y + 99.], [x + 156., y + 130.]],
        [[x + 156., y + 130.], [x + 148., y + 130.]],
        [[x + 148., y + 130.], [x + 148., y + 140.]],
        [[x + 148., y + 140.], [x + 8., y + 140.]],
        [[x + 8., y + 140.], [x + 8., y + 130.]],
        [[x + 8., y + 130.], [x, y + 130.]],
        [[x, y + 130.], [x, y + 99.]],
        [[x, y + 99.], [x + 8., y + 99.]],
        [[x + 8., y + 99.], [x + 8., y + 41.]],
        [[x + 8., y + 41.], [x, y + 41.]],
        [[x, y + 41.], [x, y + 10.]],
        [[x, y + 10.], [x + 8., y + 10.]],
        [[x + 8., y + 10.], [x + 8., y]],
    ];
}

fn stabilizer(x: f32, y: f32, s: f32) -> [[[f32; 2]; 2]; 60] {
    return [
        [[x + 8.0, y], [x + 148.0, y]], //81
        [[x + 148.0, y], [x + 148.0, y + 10.]],
        [[x + 148.0, y + 10.], [x + 156.0, y + 10.]],
        [[x + 156.0, y + 10.], [x + 156.0, y + 41.]],
        [[x + 156.0, y + 41.], [x + 148.0, y + 41.]],
        [[x + 148.0, y + 41.], [x + 148.0, y + 46.]],
        [[x + 148.0, y + 46.], [x + 163.2 + s, y + 46.]],
        [[x + 163.2 + s, y + 46.], [x + 163.2 + s, y + 14.7]],
        [[x + 163.2 + s, y + 14.7], [x + 180.4 + s, y + 14.7]],
        [[x + 180.4 + s, y + 14.7], [x + 180.4 + s, y + 5.5]],
        [[x + 180.4 + s, y + 5.5], [x + 213.4 + s, y + 5.5]],
        [[x + 213.4 + s, y + 5.5], [x + 213.4 + s, y + 14.7]],
        [[x + 213.4 + s, y + 14.7], [x + 230.6 + s, y + 14.7]],
        [[x + 230.6 + s, y + 14.7], [x + 230.6 + s, y + 46.]],
        [[x + 230.6 + s, y + 46.], [x + 238.9 + s, y + 46.]],
        [[x + 238.9 + s, y + 46.], [x + 238.9 + s, y + 74.]],
        [[x + 238.9 + s, y + 74.], [x + 230.6 + s, y + 74.]],
        [[x + 230.6 + s, y + 74.], [x + 230.6 + s, y + 136.7]],
        [[x + 230.6 + s, y + 136.7], [x + 213.4 + s, y + 136.7]],
        [[x + 213.4 + s, y + 136.7], [x + 213.4 + s, y + 146.5]],
        [[x + 213.4 + s, y + 146.5], [x + 180.4 + s, y + 146.5]],
        [[x + 180.4 + s, y + 146.5], [x + 180.4 + s, y + 136.7]],
        [[x + 180.4 + s, y + 136.7], [x + 163.2 + s, y + 136.7]],
        [[x + 163.2 + s, y + 136.7], [x + 163.2 + s, y + 92.]],
        [[x + 163.2 + s, y + 92.], [x + 148.0, y + 92.]],
        [[x + 148.0, y + 92.], [x + 148.0, y + 98.]],
        [[x + 148.0, y + 98.], [x + 156.0, y + 98.]],
        [[x + 156.0, y + 98.], [x + 156.0, y + 129.]],
        [[x + 156.0, y + 129.], [x + 148.0, y + 129.]],
        [[x + 148.0, y + 129.], [x + 148.0, y + 139.]],
        [[x + 148.0, y + 139.], [x + 8., y + 139.]],
        [[x + 8., y + 139.], [x + 8., y + 129.]],
        [[x + 8., y + 129.], [x, y + 129.]],
        [[x, y + 129.], [x, y + 98.]],
        [[x, y + 98.], [x + 8., y + 98.]],
        [[x + 8., y + 98.], [x + 8., y + 92.]],
        [[x + 8., y + 92.], [x + (-7.2) - s, y + 92.]],
        [[x + (-7.2) - s, y + 92.], [x + (-7.2) - s, y + 136.7]],
        [[x + (-7.2) - s, y + 136.7], [x + (-24.4) - s, y + 136.7]],
        [[x + (-24.4) - s, y + 136.7], [x + (-24.4) - s, y + 146.5]],
        [[x + (-24.4) - s, y + 146.5], [x + (-57.4) - s, y + 146.5]],
        [[x + (-57.4) - s, y + 146.5], [x + (-57.4) - s, y + 136.7]],
        [[x + (-57.4) - s, y + 136.7], [x + (-74.6) - s, y + 136.7]],
        [[x + (-74.6) - s, y + 136.7], [x + (-74.6) - s, y + 74.]],
        [[x + (-74.6) - s, y + 74.], [x + (-82.9) - s, y + 74.]],
        [[x + (-82.9) - s, y + 74.], [x + (-82.9) - s, y + 47.]],
        [[x + (-82.9) - s, y + 47.0], [x + (-74.6) - s, y + 47.0]],
        [[x + (-74.6) - s, y + 47.0], [x + (-74.6) - s, y + 14.7]],
        [[x + (-74.6) - s, y + 14.7], [x + (-57.4) - s, y + 14.7]],
        [[x + (-57.4) - s, y + 14.7], [x + (-57.4) - s, y + 5.5]],
        [[x + (-57.4) - s, y + 5.5], [x + (-24.4) - s, y + 5.5]],
        [[x + (-24.4) - s, y + 5.5], [x + (-24.4) - s, y + 14.7]],
        [[x + (-24.4) - s, y + 14.7], [x + (-7.2) - s, y + 14.7]],
        [[x + (-7.2) - s, y + 14.7], [x + (-7.2) - s, y + 46.]],
        [[x + (-7.2) - s, y + 46.], [x + 8.0, y + 46.]],
        [[x + 8.0, y + 46.], [x + 8.0, y + 41.]],
        [[x + 8.0, y + 41.], [x, y + 41.]],
        [[x, y + 41.], [x, y + 10.]],
        [[x, y + 10.], [x + 8.0, y + 10.]],
        [[x + 8.0, y + 10.], [x + 8.0, y]],
    ];
}

fn transform(n: [f32; 2], horizontal_offset: f32) -> [f32; 2] {
    [(n[0] + horizontal_offset) / 3., n[1] / 3.]
}

fn transform_single(n: f32) -> f32 {
    n / 3.
}
