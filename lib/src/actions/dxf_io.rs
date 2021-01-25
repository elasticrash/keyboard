use dxf::entities::*;
use dxf::Drawing;


pub fn dxf_import(geometry: &Vec<Polyline>) -> Drawing {
    let mut drawing = Drawing::default();

    for pl in geometry {
        drawing
            .entities
            .push(Entity::new(EntityType::Polyline(pl.clone())));
    }

    drawing
}

pub fn dxf_export(drawing: &Drawing, name: &String) {
    drawing.save_file(&format!("{}.dxf", name)).unwrap();
}
