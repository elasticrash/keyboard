use cgmath::Point2;
use dxf::entities::Polyline;
use dxf::entities::*;
use dxf::Drawing;
use spade::delaunay::ConstrainedDelaunayTriangulation;
use spade::kernels::FloatKernel;

pub type Cdt = ConstrainedDelaunayTriangulation<Point2<f64>, FloatKernel>;

pub fn dxf_import(geometry: &Vec<Polyline>) -> Drawing {
    let mut drawing = Drawing::default();
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

        drawing
            .entities
            .push(Entity::new(EntityType::Polyline(pl.clone())));
    }

    drawing
}

pub fn dxf_export(drawing: &Drawing, name: &String) {
    drawing.save_file(&format!("{}.dxf", name)).unwrap();
}
