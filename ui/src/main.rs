use kpt_lib::actions::dxf_io::{dxf_export, dxf_import};
use kpt_lib::actions::ply_io::{PlyExport, PlyImport, PlyObject};
use kpt_lib::config;
use kpt_lib::geometry::exported_geometry::get_geometry;
use kpt_lib::geometry::transformations;
mod virtual_keyboard;

use egaku2d::glutin::event::{Event, VirtualKeyCode, WindowEvent};
use egaku2d::glutin::event_loop::ControlFlow;
use std::env;
use virtual_keyboard::{add_ascii, generate_virtual_keyboard};

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
    let drawing = dxf_import(&geometry);

    events_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => {
                    *control_flow = ControlFlow::Exit;
                }
                Some(VirtualKeyCode::D) => {
                    dxf_export(&drawing, &name);
                }
                Some(VirtualKeyCode::P) => {
                    let mut ply = PlyObject::new();
                    ply.import(&keyboard, &geometry);
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