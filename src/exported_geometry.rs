use crate::config::model::Layout;
use dxf::entities::Polyline;
use dxf::entities::Vertex;
use dxf::Point;

pub fn get_geometry(config: &Layout) -> Vec<Polyline> {
    let mut polylines: Vec<Polyline> = vec![];
    let spacing = 190.;
    let y_starting_point = 500.;
    let mut y_position = 500.;
    let mut minimum_board_width = 0.;
    let mut row_num: i32 = 0;
    let border = 50.0;
    for row in &config.layout {
        let mut x_position = border;
        row_num += 1;
        for key in row {
            // this is used for keys bigger than 1U to be placed correctly
            // for 1U keys this results to 0
            let offset: f64 = (((key.size - 1.) * spacing) / 2.) as f64;

            if key.k_type == 1 {
                if key.size < 2. {
                    polylines.push(switch(x_position + offset, y_position));
                }
                if key.size == 6.25 {
                    polylines.push(stabilizer(x_position + offset, y_position, 381.5));
                }
                if key.size > 2. {
                    polylines.push(stabilizer(x_position + offset, y_position, 0.));
                }
            }
            x_position += 190.5 + offset * 2.;

            if row_num == 1 {
                minimum_board_width = minimum_board_width + spacing * key.size;
            }
        }
        y_position -= spacing as f64 + 0.5;
    }

    polylines.push(frame(
        minimum_board_width as f64,
        (((row_num as f32 * spacing) + ((row_num - 1) as f32 * 0.5)) - 50.) as f64,
        border,
        y_starting_point,
        border,
    ));

    return polylines;
}

fn frame(
    minimum_board_width: f64,
    minimum_board_height: f64,
    x: f64,
    y: f64,
    border: f64,
) -> Polyline {
    let mut polyline = Polyline::default();

    polyline.vertices = [
        Vertex::new(Point::new(x - border, y + border, 0.)),
        Vertex::new(Point::new(x + minimum_board_width + border, y + border, 0.)),
        Vertex::new(Point::new(
            x + minimum_board_width + border,
            y - (minimum_board_height + border),
            0.,
        )),
        Vertex::new(Point::new(
            x - border,
            y - (minimum_board_height + border),
            0.,
        )),
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
