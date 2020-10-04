extern crate egaku2d;
use egaku2d::glutin::event::{Event, VirtualKeyCode, WindowEvent};
use egaku2d::glutin::event_loop::ControlFlow;
mod config;
use std::env;

fn main() {
    let events_loop = egaku2d::glutin::event_loop::EventLoop::new();
    let mut sys = egaku2d::WindowedSystem::new([1200, 1000], &events_loop, "keyboard tester");
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
                Some(VirtualKeyCode::Key1) => {
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
                let mut sprites = canvas.sprites();
                let mut lines = canvas.lines(1.3);

                let mut row = 1;
                let mut pos_y = 50.;
                let u1_size = 50.;
                let gap = 65.;
                let ff = 0.1;

                for x in &keyboard.layout {
                    let mut point = 1.0;
                    let mut pos_x = 50.;
                    for y in x {
                        rects.add(rectangle(pos_x, pos_y, u1_size, row, point, y.size));
                        add_ascii(
                            [
                                pos_x + (point * u1_size) + u1_size / 3. as f32,
                                pos_y + ((row as f32) * u1_size) + u1_size / 3. as f32,
                            ],
                            10.0,
                            0.0,
                            &y.display,
                            &mut sprites,
                        );
                        point += y.size;
                        pos_x += gap / x.len() as f32;
                    }
                    pos_y += 5.;
                    row += 1;
                    rects
                        .send_and_uniforms(canvas)
                        .with_color([ff * row as f32, 0.6, 0.4, 1.0])
                        .draw();
                    rects = canvas.rects();
                }

                row += 3;
                pos_y = 500.;

                let mut pos_x = 50.;
                lines.add([pos_x, pos_y], [pos_x + 8., pos_y]);
                lines.add([pos_x + 8., pos_y], [pos_x + 8., pos_y - 10.]);
                lines.add([pos_x + 8., pos_y - 10.], [pos_x + 148., pos_y - 10.]);
                lines.add([pos_x + 148., pos_y - 10.], [pos_x + 148., pos_y]);
                lines.add([pos_x + 148., pos_y], [pos_x + 156., pos_y]);
                lines.add([pos_x + 156., pos_y], [pos_x + 156., pos_y + 31.]);
                lines.add([pos_x + 156., pos_y + 31.], [pos_x + 148., pos_y + 31.]);
                lines.add([pos_x + 148., pos_y + 31.], [pos_x + 148., pos_y + 89.]);
                lines.add([pos_x + 148., pos_y + 89.], [pos_x + 156., pos_y + 89.]);
                lines.add([pos_x + 156., pos_y + 89.], [pos_x + 156., pos_y + 120.]);
                lines.add([pos_x + 156., pos_y + 120.], [pos_x + 148., pos_y + 120.]);
                lines.add([pos_x + 148., pos_y + 120.], [pos_x + 148., pos_y + 130.]);
                lines.add([pos_x + 148., pos_y + 130.], [pos_x + 8., pos_y + 130.]);
                lines.add([pos_x + 8., pos_y + 130.], [pos_x + 8., pos_y + 120.]);
                lines.add([pos_x + 8., pos_y + 120.], [pos_x, pos_y + 120.]);
                lines.add([pos_x, pos_y + 120.], [pos_x, pos_y + 89.]);
                lines.add([pos_x, pos_y + 89.], [pos_x + 8., pos_y + 89.]);
                lines.add([pos_x + 8., pos_y + 89.], [pos_x + 8., pos_y + 31.]);
                lines.add([pos_x + 8., pos_y + 31.], [pos_x, pos_y + 31.]);
                lines.add([pos_x, pos_y + 31.], [pos_x, pos_y]);

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

fn rectangle(x: f32, y: f32, s: f32, r: i32, p: f32, key: f32) -> [f32; 4] {
    return [
        x + (p * s) as f32,
        x + (p * s + (key * s)) as f32,
        y + ((r as f32) * s) as f32,
        y + ((r as f32) * s + s) as f32,
    ];
}
