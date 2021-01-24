use ply_rs::ply::{
    Addable, DefaultElement, ElementDef, Encoding, Ply, PropertyDef, PropertyType,
    ScalarType,
};
use ply_rs::writer::Writer;
use std::fs::File;
use std::io::Write;

pub trait PlyExport {
    fn new() -> Self;
    fn create_vertex_header(&mut self);
    fn create_triangle_header(&mut self);
    fn write_to_file(&mut self, name: String);
    fn add_enconding(&mut self, encoding: Encoding);
    fn add_comment(&mut self, comment: String);
}

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
