extern crate egaku2d;
extern crate ply_rs;
extern crate spade;

mod actions;
mod config;
mod exported_geometry;
mod primitives;
mod transformations;
mod virtual_keyboard;

use crate::actions::ply_export::{PlyExport, PlyObject};
use crate::exported_geometry::get_geometry;
use crate::primitives::add_ascii;
use crate::primitives::point_in_polygon;
use crate::primitives::{Find, TriangleIndex, VertexIndex};
use crate::virtual_keyboard::generate_virtual_keyboard;

use dxf::entities::*;
use dxf::Drawing;
use egaku2d::glutin::event::{Event, VirtualKeyCode, WindowEvent};
use egaku2d::glutin::event_loop::ControlFlow;
use ply_rs::ply::{DefaultElement, Encoding, Property};
use spade::kernels::FloatKernel;
use std::env;

use cgmath::Point2;
use spade::delaunay::ConstrainedDelaunayTriangulation;

pub type Cdt = ConstrainedDelaunayTriangulation<Point2<f64>, FloatKernel>;




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

    let mut cdt = Cdt::new();

    for pl in &geometry {
        for vtx in 0..pl.vertices.len() {
            if vtx + 1 < pl.vertices.len() {
                let a = cdt.insert(Point2::new(
                    pl.vertices[vtx].location.x,
                    pl.vertices[vtx].location.y,
                ));
                let b = cdt.insert(Point2::new(
                    pl.vertices[vtx + 1].location.x,
                    pl.vertices[vtx + 1].location.y,
                ));
                cdt.add_constraint(a, b);
            }
        }

        drawing
            .entities
            .push(Entity::new(EntityType::Polyline(pl.clone())));
    }

    events_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => {
                    *control_flow = ControlFlow::Exit;
                }
                Some(VirtualKeyCode::D) => {
                    drawing.save_file(&format!("{}.dxf", name)).unwrap();
                }
                Some(VirtualKeyCode::P) => {
                    let mut points = Vec::new();
                    let mut triangles = Vec::new();
                    let mut original_triangles: Vec<TriangleIndex> = Vec::new();
                    let mut ply = PlyObject::new();
                    ply.add_enconding(Encoding::Ascii);
                    ply.create_vertex_header();

                    for tr in cdt.vertices() {
                        let mut point = DefaultElement::new();
                        point.insert("x".to_string(), Property::Float(tr.x as f32));
                        point.insert("y".to_string(), Property::Float(tr.y as f32));
                        point.insert("z".to_string(), Property::Float(0.));
                        points.push(point);
                    }
                    for tr in cdt.vertices() {
                        let mut point = DefaultElement::new();
                        point.insert("x".to_string(), Property::Float(tr.x as f32));
                        point.insert("y".to_string(), Property::Float(tr.y as f32));
                        point.insert("z".to_string(), Property::Float(keyboard.options.plate_height as f32));
                        points.push(point);
                    }

                    let points_len = points.len();
                    ply.object.payload.insert("vertex".to_string(), points);
                    ply.object.make_consistent().unwrap();

                    for tr in cdt.triangles() {
                        let mut triangle = DefaultElement::new();
                        let trngl = tr.as_triangle();
                        let mut pip = false;

                        let point_avg_x = (trngl[0][0] + trngl[1][0] + trngl[2][0]) / 3.;
                        let point_avg_y = (trngl[0][1] + trngl[1][1] + trngl[2][1]) / 3.;

                        for pl in &geometry {
                            if pl.vertices.len() > 5 {
                                if point_in_polygon(pl.vertices.to_vec(), point_avg_x, point_avg_y)
                                {
                                    pip = true;
                                    break;
                                }
                            }
                        }
                        if pip == false {
                            triangle.insert(
                                "vertex_index".to_string(),
                                Property::ListInt(vec![
                                    trngl[0].fix() as i32,
                                    trngl[1].fix() as i32,
                                    trngl[2].fix() as i32,
                                ]),
                            );
                            triangles.push(triangle);
                            original_triangles.push(TriangleIndex {
                                a: VertexIndex {
                                    v: trngl[0].fix() as i32,
                                    x: trngl[0][0] as f32,
                                    y: trngl[0][1] as f32,
                                    z: 0.,
                                },
                                b: VertexIndex {
                                    v: trngl[1].fix() as i32,
                                    x: trngl[1][0] as f32,
                                    y: trngl[1][1] as f32,
                                    z: 0.,
                                },
                                c: VertexIndex {
                                    v: trngl[2].fix() as i32,
                                    x: trngl[2][0] as f32,
                                    y: trngl[2][1] as f32,
                                    z: 0.,
                                },
                            })
                        }
                    }

                    for tr in &original_triangles {
                        let mut triangle = DefaultElement::new();

                        triangle.insert(
                            "vertex_index".to_string(),
                            Property::ListInt(vec![
                                tr.a.v + (points_len / 2) as i32,
                                tr.b.v + (points_len / 2) as i32,
                                tr.c.v + (points_len / 2) as i32,
                            ]),
                        );
                        triangles.push(triangle);
                    }

                    for pl in &geometry {
                        for vtx in 0..pl.vertices.len() {
                            if vtx + 1 < pl.vertices.len() {
                                let mut find_first = -1;

                                for ort in original_triangles.to_vec() {
                                    find_first = ort.search(
                                        pl.vertices[vtx].location.x as f32,
                                        pl.vertices[vtx].location.y as f32,
                                    );

                                    if find_first != -1 {
                                        break;
                                    }
                                }

                                let mut find_second = -1;

                                for ort in original_triangles.to_vec() {
                                    find_second = ort.search(
                                        pl.vertices[vtx + 1].location.x as f32,
                                        pl.vertices[vtx + 1].location.y as f32,
                                    );
                                    if find_second != -1 {
                                        break;
                                    }
                                }

                                if find_first != -1 && find_second != -1 {
                                    let mut triangle_a = DefaultElement::new();

                                    triangle_a.insert(
                                        "vertex_index".to_string(),
                                        Property::ListInt(vec![
                                            find_first + (points_len / 2) as i32,
                                            find_first,
                                            find_second,
                                        ]),
                                    );
                                    triangles.push(triangle_a);

                                    let mut triangle_b = DefaultElement::new();

                                    triangle_b.insert(
                                        "vertex_index".to_string(),
                                        Property::ListInt(vec![
                                            find_first + (points_len / 2) as i32,
                                            find_second,
                                            find_second + (points_len / 2) as i32,
                                        ]),
                                    );
                                    triangles.push(triangle_b);
                                }
                            }
                        }
                    }

                    ply.create_triangle_header();
                    ply.object.payload.insert("face".to_string(), triangles);
                    ply.object.make_consistent().unwrap();
                    ply.write_to_file(name.to_string());
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

                let mut sprites = canvas.sprites();

                add_ascii([10., 10.], 10.0, 0.0, "D: exports dxf", &mut sprites);
                add_ascii([10., 20.], 10.0, 0.0, "P: exports ply", &mut sprites);

                generate_virtual_keyboard(canvas, &mut sprites, &keyboard, &geometry);

                sprites.send_and_uniforms(canvas, &ascii_tex, 10.0).draw();

                //display what we drew
                sys.swap_buffers();
            }
        }
        _ => {}
    });
}
