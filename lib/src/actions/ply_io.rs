use crate::geometry::primitives::*;
use crate::config::model::Layout;
use cgmath::Point2;
use dxf::entities::*;
use ply_rs::ply::{
    Addable, DefaultElement, ElementDef, Encoding, Ply, Property, PropertyDef, PropertyType,
    ScalarType,
};
use ply_rs::writer::Writer;
use spade::delaunay::ConstrainedDelaunayTriangulation;
use spade::kernels::FloatKernel;
use std::fs::File;
use std::io::Write;

pub type Cdt = ConstrainedDelaunayTriangulation<Point2<f64>, FloatKernel>;

pub trait PlyExport {
    fn new() -> Self;
    fn create_vertex_header(&mut self);
    fn create_triangle_header(&mut self);
    fn write_to_file(&mut self, name: String);
    fn add_enconding(&mut self, encoding: Encoding);
    fn add_comment(&mut self, comment: String);
}

pub trait PlyImport {
    fn import(&mut self, keyboard: &Layout, geometry: &Vec<Polyline>);
}

#[derive(Debug)]
pub struct PlyObject {
    pub object: Ply<DefaultElement>,
}

impl PlyExport for PlyObject {
    fn new() -> Self {
        Self {
            object: Ply::<DefaultElement>::new(),
        }
    }
    fn create_vertex_header(&mut self) {
        let mut point_element = ElementDef::new("vertex".to_string());
        let p = PropertyDef::new("x".to_string(), PropertyType::Scalar(ScalarType::Float));
        point_element.properties.add(p);
        let p = PropertyDef::new("y".to_string(), PropertyType::Scalar(ScalarType::Float));
        point_element.properties.add(p);
        let p = PropertyDef::new("z".to_string(), PropertyType::Scalar(ScalarType::Float));
        point_element.properties.add(p);
        self.object.header.elements.add(point_element);
    }
    fn create_triangle_header(&mut self) {
        let mut face_element = ElementDef::new("face".to_string());
        let p = PropertyDef::new(
            "vertex_index".to_string(),
            PropertyType::List(ScalarType::UChar, ScalarType::Int),
        );
        face_element.properties.add(p);
        self.object.header.elements.add(face_element);
    }
    fn write_to_file(&mut self, name: String) {
        let mut buf = Vec::<u8>::new();
        let w = Writer::new();
        w.write_ply(&mut buf, &mut self.object).unwrap();
        let mut file = File::create(format!("{}.ply", name)).unwrap();
        file.write_all(&buf).unwrap();
    }
    fn add_enconding(&mut self, encoding: Encoding) {
        self.object.header.encoding = encoding;
    }
    fn add_comment(&mut self, comment: String) {
        self.object.header.comments.push(comment);
    }
}

impl PlyImport for PlyObject {
    fn import(&mut self, keyboard: &Layout, geometry: &Vec<Polyline>) {
        let mut cdt = Cdt::new();

        for pl in geometry {
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
        }

        let mut points = Vec::new();
        let mut triangles = Vec::new();
        let mut original_triangles: Vec<TriangleIndex> = Vec::new();
        self.add_enconding(Encoding::Ascii);
        self.create_vertex_header();

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
            point.insert(
                "z".to_string(),
                Property::Float(keyboard.options.plate_height as f32),
            );
            points.push(point);
        }

        let points_len = points.len();
        self.object.payload.insert("vertex".to_string(), points);
        self.object.make_consistent().unwrap();

        for tr in cdt.triangles() {
            let mut triangle = DefaultElement::new();
            let trngl = tr.as_triangle();
            let mut pip = false;

            let point_avg_x = (trngl[0][0] + trngl[1][0] + trngl[2][0]) / 3.;
            let point_avg_y = (trngl[0][1] + trngl[1][1] + trngl[2][1]) / 3.;

            for pl in geometry {
                if pl.vertices.len() > 5 {
                    if point_in_polygon(pl.vertices.to_vec(), point_avg_x, point_avg_y) {
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

        for pl in geometry {
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

        
        self.create_triangle_header();
        self.object.payload.insert("face".to_string(), triangles);
        self.object.make_consistent().unwrap();
    }
}
