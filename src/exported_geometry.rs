use crate::config::model::Layout;
use dxf::entities::Polyline;
use dxf::entities::Vertex;
use dxf::Point;
extern crate orchestrator;
use orchestrator::{state_function, Chain, Error, Orchestrate, Register, Registry, State};

pub struct Output<'a> {
    polylines: Vec<Polyline>,
    config: &'a Layout,
    passed: Passed,
}

struct Passed {
    row_num: f32,
    board_width: f32,
}

pub fn get_geometry(config: &Layout) -> Vec<Polyline> {
    let fn1: fn(State<Output>) -> Result<State<Output>, Error> = state_function!(switches, Output);
    let fn2: fn(State<Output>) -> Result<State<Output>, Error> = state_function!(screws, Output);
    let fn3: fn(State<Output>) -> Result<State<Output>, Error> = state_function!(add_frame, Output);

    let mut registry = Registry::new();

    registry.register(fn1, "sw".to_string());
    registry.register(fn2, "sc".to_string());
    registry.register(fn3, "fr".to_string());

    let polylines: Vec<Polyline> = vec![];

    let result = vec!["sw", "sc", "fr"]
        .create(&registry.di_ref)
        .execute(State {
            proceed: true,
            outcome: Some(Output {
                polylines: polylines,
                config: &config,
                passed: Passed {
                    row_num: 0.,
                    board_width: 0.,
                },
            }),
            stage: Vec::<bool>::new(),
        });

    result.outcome.unwrap().polylines
}

fn switches(mut out: Output) -> Option<Output> {
    let spacing = 190.; // should go on config
    let mut y_position = 500.;
    let mut board_width = 0.;
    let mut row_num: i32 = 0;
    let border = 50.0; // should go on config
    for row in &out.config.layout {
        let mut x_position = border;
        row_num += 1;
        for key in row {
            // this is used for keys bigger than 1U to be placed correctly
            // for 1U keys this results to 0
            let offset: f64 = (((key.size - 1.) * spacing) / 2.) as f64;

            if key.k_type == 1 {
                if key.size < 2. {
                    out.polylines.push(switch(x_position + offset, y_position));
                } else if key.size == 6.25 {
                    out.polylines
                        .push(stabilizer(x_position + offset, y_position, 381.5));
                } else if key.size >= 2. {
                    out.polylines
                        .push(stabilizer(x_position + offset, y_position, 0.));
                }
            }
            x_position += 190.5 + offset * 2.;

            if row_num == 1 {
                board_width = board_width + spacing * key.size;
            }
        }
        y_position -= spacing as f64 + 0.5;
    }

    out.passed.row_num = row_num as f32;
    out.passed.board_width = board_width;

    Some(out)
}

fn screws(mut out: Output) -> Option<Output> {
    let spacing = 190.; // should go on config
    let border = 50.0; // should go on config
    let y_starting_point = 500.; // should go on config

    let board_height =
        (((out.passed.row_num * spacing) + ((out.passed.row_num - 1.) * 0.5)) - 50.) as f64;

    if out.config.options.screw_holes {
        println!("drilling holes");
        out.polylines.push(screw(
            border / 2.,
            y_starting_point + (border / 2.),
            10.,
            20,
        ));
        out.polylines.push(screw(
            border / 2.,
            y_starting_point - (border / 2.) - board_height,
            10.,
            20,
        ));
        out.polylines.push(screw(
            out.passed.board_width as f64 + border,
            y_starting_point + (border / 2.),
            10.,
            20,
        ));
        out.polylines.push(screw(
            out.passed.board_width as f64 + border,
            y_starting_point - (border / 2.) - board_height,
            10.,
            20,
        ));
    }

    Some(out)
}

fn add_frame(mut out: Output) -> Option<Output> {
    let border = 50.0; // should go on config
    let spacing = 190.; // should go on config
    let y_starting_point = 500.; // should go on config

    let board_height =
        (((out.passed.row_num * spacing) + ((out.passed.row_num - 1.) * 0.5)) - 50.) as f64;
    out.polylines.push(frame(
        out.passed.board_width as f64 - 25.,
        board_height,
        border,
        y_starting_point,
        border,
    ));

    Some(out)
}

fn frame(board_width: f64, board_height: f64, x: f64, y: f64, border: f64) -> Polyline {
    let mut polyline = Polyline::default();

    polyline.vertices = [
        Vertex::new(Point::new(x - border, y + border, 0.)),
        Vertex::new(Point::new(x + board_width + border, y + border, 0.)),
        Vertex::new(Point::new(
            x + board_width + border,
            y - (board_height + border),
            0.,
        )),
        Vertex::new(Point::new(x - border, y - (board_height + border), 0.)),
        Vertex::new(Point::new(x - border, y + border, 0.)),
    ]
    .to_vec();

    polyline
}

fn switch(x: f64, y: f64) -> Polyline {
    let mut polyline = Polyline::default();

    polyline.vertices = [
        Vertex::new(Point::new(x + 8., y, 0.)),
        Vertex::new(Point::new(x + 148., y, 0.)),
        Vertex::new(Point::new(x + 148., y - 10., 0.)),
        Vertex::new(Point::new(x + 156., y - 10., 0.)),
        Vertex::new(Point::new(x + 156., y - 41., 0.)),
        Vertex::new(Point::new(x + 148., y - 41., 0.)),
        Vertex::new(Point::new(x + 148., y - 99., 0.)),
        Vertex::new(Point::new(x + 156., y - 99., 0.)),
        Vertex::new(Point::new(x + 156., y - 130., 0.)),
        Vertex::new(Point::new(x + 148., y - 130., 0.)),
        Vertex::new(Point::new(x + 148., y - 140., 0.)),
        Vertex::new(Point::new(x + 8., y - 140., 0.)),
        Vertex::new(Point::new(x + 8., y - 130., 0.)),
        Vertex::new(Point::new(x, y - 130., 0.)),
        Vertex::new(Point::new(x, y - 99., 0.)),
        Vertex::new(Point::new(x + 8., y - 99., 0.)),
        Vertex::new(Point::new(x + 8., y - 41., 0.)),
        Vertex::new(Point::new(x, y - 41., 0.)),
        Vertex::new(Point::new(x, y - 10., 0.)),
        Vertex::new(Point::new(x + 8., y - 10., 0.)),
        Vertex::new(Point::new(x + 8., y, 0.)),
    ]
    .to_vec();

    polyline
}

fn stabilizer(x: f64, y: f64, s: f64) -> Polyline {
    let mut polyline = Polyline::default();

    polyline.vertices = [
        Vertex::new(Point::new(x + 8., y, 0.)),
        Vertex::new(Point::new(x + 148., y, 0.)),
        Vertex::new(Point::new(x + 148., y - 10., 0.)),
        Vertex::new(Point::new(x + 156., y - 10., 0.)),
        Vertex::new(Point::new(x + 156., y - 41., 0.)),
        Vertex::new(Point::new(x + 148., y - 41., 0.)),
        Vertex::new(Point::new(x + 148.0, y - 46., 0.)),
        Vertex::new(Point::new(x + 163.2 + s, y - 46., 0.)),
        Vertex::new(Point::new(x + 163.2 + s, y - 14.7, 0.)),
        Vertex::new(Point::new(x + 180.4 + s, y - 14.7, 0.)),
        Vertex::new(Point::new(x + 180.4 + s, y - 5.5, 0.)),
        Vertex::new(Point::new(x + 213.4 + s, y - 5.5, 0.)),
        Vertex::new(Point::new(x + 213.4 + s, y - 14.7, 0.)),
        Vertex::new(Point::new(x + 230.6 + s, y - 14.7, 0.)),
        Vertex::new(Point::new(x + 230.6 + s, y - 46., 0.)),
        Vertex::new(Point::new(x + 238.9 + s, y - 46., 0.)),
        Vertex::new(Point::new(x + 238.9 + s, y - 74., 0.)),
        Vertex::new(Point::new(x + 230.6 + s, y - 74., 0.)),
        Vertex::new(Point::new(x + 230.6 + s, y - 136.7, 0.)),
        Vertex::new(Point::new(x + 213.4 + s, y - 136.7, 0.)),
        Vertex::new(Point::new(x + 213.4 + s, y - 146.5, 0.)),
        Vertex::new(Point::new(x + 180.4 + s, y - 146.5, 0.)),
        Vertex::new(Point::new(x + 180.4 + s, y - 136.7, 0.)),
        Vertex::new(Point::new(x + 163.2 + s, y - 136.7, 0.)),
        Vertex::new(Point::new(x + 163.2 + s, y - 92., 0.)),
        Vertex::new(Point::new(x + 148.0, y - 92., 0.)),
        Vertex::new(Point::new(x + 148.0, y - 98., 0.)),
        Vertex::new(Point::new(x + 156.0, y - 98., 0.)),
        Vertex::new(Point::new(x + 156.0, y - 129., 0.)),
        Vertex::new(Point::new(x + 148.0, y - 129., 0.)),
        Vertex::new(Point::new(x + 148.0, y - 139., 0.)),
        Vertex::new(Point::new(x + 8., y - 139., 0.)),
        Vertex::new(Point::new(x + 8., y - 129., 0.)),
        Vertex::new(Point::new(x, y - 129., 0.)),
        Vertex::new(Point::new(x, y - 98., 0.)),
        Vertex::new(Point::new(x + 8., y - 98., 0.)),
        Vertex::new(Point::new(x + 8., y - 92., 0.)),
        Vertex::new(Point::new(x + (-7.2) - s, y - 92., 0.)),
        Vertex::new(Point::new(x + (-7.2) - s, y - 136.7, 0.)),
        Vertex::new(Point::new(x + (-24.4) - s, y - 136.7, 0.)),
        Vertex::new(Point::new(x + (-24.4) - s, y - 146.5, 0.)),
        Vertex::new(Point::new(x + (-57.4) - s, y - 146.5, 0.)),
        Vertex::new(Point::new(x + (-57.4) - s, y - 136.7, 0.)),
        Vertex::new(Point::new(x + (-74.6) - s, y - 136.7, 0.)),
        Vertex::new(Point::new(x + (-74.6) - s, y - 74., 0.)),
        Vertex::new(Point::new(x + (-82.9) - s, y - 74., 0.)),
        Vertex::new(Point::new(x + (-82.9) - s, y - 47.0, 0.)),
        Vertex::new(Point::new(x + (-74.6) - s, y - 47.0, 0.)),
        Vertex::new(Point::new(x + (-74.6) - s, y - 14.7, 0.)),
        Vertex::new(Point::new(x + (-57.4) - s, y - 14.7, 0.)),
        Vertex::new(Point::new(x + (-57.4) - s, y - 5.5, 0.)),
        Vertex::new(Point::new(x + (-24.4) - s, y - 5.5, 0.)),
        Vertex::new(Point::new(x + (-24.4) - s, y - 14.7, 0.)),
        Vertex::new(Point::new(x + (-7.2) - s, y - 14.7, 0.)),
        Vertex::new(Point::new(x + (-7.2) - s, y - 46., 0.)),
        Vertex::new(Point::new(x + 8.0, y - 46., 0.)),
        Vertex::new(Point::new(x + 8.0, y - 41., 0.)),
        Vertex::new(Point::new(x, y - 41., 0.)),
        Vertex::new(Point::new(x, y - 10., 0.)),
        Vertex::new(Point::new(x + 8.0, y - 10., 0.)),
        Vertex::new(Point::new(x + 8., y, 0.)),
    ]
    .to_vec();

    polyline
}

fn screw(x: f64, y: f64, r: f64, s: i32) -> Polyline {
    let mut polyline = Polyline::default();
    let single_segment = std::f64::consts::PI * 2. / (s as f64);
    let i = 0;

    for it in i..=s {
        polyline.vertices.push(Vertex::new(Point::new(
            x + (single_segment * it as f64).cos() * r,
            y + (single_segment * it as f64).sin() * r,
            0.,
        )));
    }

    polyline
}
