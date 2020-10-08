extern crate egaku2d;
use egaku2d::glutin::event::{Event, VirtualKeyCode, WindowEvent};
use egaku2d::glutin::event_loop::ControlFlow;
mod config;
use std::env;
use std::iter::Repeat;

fn main() {
    let events_loop = egaku2d::glutin::event_loop::EventLoop::new();
    let mut sys = egaku2d::WindowedSystem::new([1200, 1000], &events_loop, "keyboard designer");
    let args: Vec<String> = env::args().collect();
    let file: &str = args[1].as_ref();
    let ascii_tex = sys.texture("ascii.png", [16, 14]).unwrap();

    let keyboard = config::reader::read(format!("{}.json", file).as_str()).unwrap();

    //Draw 60 frames per second.
    let mut timer = egaku2d::RefreshTimer::new(16);

    events_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => {
                    *control_flow = ControlFlow::Exit;
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

                for x in &keyboard.layout {
                    let mut point = 1.0;
                    let mut pos_x = 50.;
                    for y in x {
                        rects.add(rectangle(pos_x, pos_y, u1_size, row, point, y.size, 0.0));
                        rects_inner.add(rectangle(pos_x, pos_y, u1_size, row, point, y.size, 1.));

                        add_ascii(
                            [
                                transform_single(pos_x + (point * u1_size) + u1_size / 3. as f32),
                                transform_single(
                                    pos_y + ((row as f32) * u1_size) + u1_size / 3. as f32,
                                ),
                            ],
                            10.0,
                            0.0,
                            &y.display,
                            &mut sprites,
                        );
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
                        let switch = switch_slot(pos_x, pos_y);
                        let offset = ((y.size - 1.) * 190.) / 2.;
                        for swln_index in 0..switch.len() {
                            lines.add(
                                transform(switch[swln_index][0], offset),
                                transform(switch[swln_index][1], offset),
                            );
                        }
                        pos_x += 190.5 + offset * 2.;
                    }
                    pos_y += 190.5;
                }

                let stabilizer = stabilizer(50.0, 50.0, 0.0);
                for swln_index in 0..stabilizer.len() {
                    lines.add(stabilizer[swln_index][0], stabilizer[swln_index][1]);
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

fn stabilizer(x: f32, y: f32, s: f32) -> [[[f32; 2]; 2]; 24] {
    return [
        [[x + 25.5, y + 5.5], [x + 58.5, y + 5.5]],
        [[x + 58.5, y + 5.5], [x + 58.5, y + 14.7]],
        [[x + 58.5, y + 14.7], [x + 75.7, y + 14.7]],
        [[x + 75.7, y + 14.7], [x + 75.7, y + 46.]],
        [[x + 75.7, y + 46.], [x + 89.0 + s, y + 46.]],
        [[x + 89.0 + s, y + 46.], [x + 89.0, y + 41.]],
        [[x + 89.0, y + 41.], [x + 81.0, y + 41.]],
        [[x + 81.0, y + 41.], [x + 81.0, y + 10.]],
        [[x + 81.0, y + 10.], [x + 89.0, y + 10.]],
        [[x + 89.0, y + 10.], [x + 89.0, y]],
        [[x + 89.0, y], [x + 229.0, y]],
        [[x + 229.0, y], [x + 229.0, y + 10.]],
        [[x + 229.0, y + 10.], [x + 237.0, y + 10.]],
        [[x + 237.0, y + 10.], [x + 237.0, y + 41.]],
        [[x + 237.0, y + 41.], [x + 229.0, y + 41.]],
        [[x + 229.0, y + 41.], [x + 229.0, y + 46.]],
        [[x + 229.0, y + 46.], [x + 242.3, y + 46.]],
        [[x + 242.3, y + 46.], [x + 242.3, y + 14.7]],
        [[x + 242.3, y + 14.7], [x + 267.8, y + 14.7]],
        [[x + 267.8, y + 14.7], [x + 267.8, y + 5.5]],
        
        [[x, y + 47.0], [x + 8.3, y + 47.0]],
        [[x + 8.3, y + 47.0], [x + 8.3, y + 14.7]],
        [[x + 8.3, y + 14.7], [x + 25.5, y + 14.7]],
        [[x + 25.5, y + 14.7], [x + 25.5, y + 5.5]],
    ];
}

fn transform(n: [f32; 2], horizontal_offset: f32) -> [f32; 2] {
    [(n[0] + horizontal_offset) / 3., n[1] / 3.]
}

fn transform_single(n: f32) -> f32 {
    n / 3.
}
