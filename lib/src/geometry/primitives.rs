use dxf::entities::Vertex;

#[derive(Clone, Debug)]
pub struct TriangleIndex {
    pub a: VertexIndex,
    pub b: VertexIndex,
    pub c: VertexIndex,
}

#[derive(Clone, Debug)]
pub struct VertexIndex {
    pub v: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait Find {
    fn search(self, x: f32, y: f32) -> i32;
}

impl Find for TriangleIndex {
    fn search(self, x: f32, y: f32) -> i32 {
        if self.a.x == x && self.a.y == y {
            return self.a.v;
        }
        if self.b.x == x && self.b.y == y {
            return self.b.v;
        }
        if self.c.x == x && self.c.y == y {
            return self.c.v;
        }
        return -1;
    }
}

pub fn point_in_polygon(poly: Vec<Vertex>, x: f64, y: f64) -> bool {
    let mut c = false;
    let l = poly.len() as i32;
    let mut i: i32 = -1;
    let mut j: i32 = l - 1;

    while {
        i += 1;
        i < l
    } {
        if ((poly[i as usize].location.y <= y && y < poly[j as usize].location.y)
            || (poly[j as usize].location.y <= y && y < poly[i as usize].location.y))
            && (x
                < (poly[j as usize].location.x - poly[i as usize].location.x)
                    * (y - poly[i as usize].location.y)
                    / (poly[j as usize].location.y - poly[i as usize].location.y)
                    + poly[i as usize].location.x)
        {
            c = !c;
        }
        j = i;
    }
    c == true
}