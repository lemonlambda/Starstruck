pub mod vertex;

pub use crate::shapes::vertex::Vertex;

#[derive(Clone)]
pub struct Triangle {
    vertices: [Vertex; 3],
}

impl Triangle {
    pub fn new(
        p1: (f32, f32, f32, f32),
        p2: (f32, f32, f32, f32),
        p3: (f32, f32, f32, f32),
        color: (f32, f32, f32, f32),
    ) -> Self {
        Self {
            vertices: [
                Vertex {
                    position: p1.into(),
                    color: color.into(),
                },
                Vertex {
                    position: p2.into(),
                    color: color.into(),
                },
                Vertex {
                    position: p3.into(),
                    color: color.into(),
                },
            ],
        }
    }
    pub fn rotate_around_center(mut self, pitch: f32, roll: f32, yaw: f32) -> Self {
        // Centroid
        let x1 = self.vertices[0].position[0];
        let x2 = self.vertices[1].position[0];
        let x3 = self.vertices[2].position[0];
        let y1 = self.vertices[0].position[1];
        let y2 = self.vertices[1].position[1];
        let y3 = self.vertices[2].position[1];
        let z1 = self.vertices[0].position[2];
        let z2 = self.vertices[1].position[2];
        let z3 = self.vertices[2].position[2];
        let centroid = (
            (x1 + x2 + x3) / 3.0,
            (y1 + y2 + y3) / 3.0,
            (z1 + z2 + z3) / 3.0,
        );

        self.rotate_around(pitch, roll, yaw, centroid)
    }

    pub fn rotate_around(mut self, pitch: f32, roll: f32, yaw: f32, p: (f32, f32, f32)) -> Self {
        self.vertices[0] = self.vertices[0].rotate_around(pitch, roll, yaw, p);
        self.vertices[1] = self.vertices[1].rotate_around(pitch, roll, yaw, p);
        self.vertices[2] = self.vertices[2].rotate_around(pitch, roll, yaw, p);
        self
    }

    pub fn rotate(mut self, pitch: f32, roll: f32, yaw: f32) -> Self {
        self.vertices[0] = self.vertices[0].rotate(pitch, roll, yaw);
        self.vertices[1] = self.vertices[1].rotate(pitch, roll, yaw);
        self.vertices[2] = self.vertices[2].rotate(pitch, roll, yaw);
        self
    }

    pub fn into_raw(self) -> (Vec<Vertex>, u32) {
        (self.vertices.to_vec(), 3)
    }
}

// pub const VERTICES: &[Vertex] = &[
//     Vertex { // top
//         position: [0.0, 0.5, 0.0, 1.0],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex { // bottom left
//         position: [-0.5, -0.5, 0.0, 1.0],
//         color: [0.0, 1.0, 0.0],
//     },
//     Vertex { // bottom right
//         position: [0.0, -0.5, 0.0, 1.0],
//         color: [0.0, 0.0, 1.0],
//     },
// ];
#[derive(Clone)]
pub struct Rectangle {
    triangles: Vec<Triangle>,
}

impl Rectangle {
    pub fn new(
        (c1x, c1y, c1z): (f32, f32, f32),
        (c2x, c2y, c2z): (f32, f32, f32),
        w: f32,
        color1: (f32, f32, f32, f32),
        color2: (f32, f32, f32, f32),
    ) -> Self {
        Self {
            triangles: vec![
                Triangle::new(
                    (c1x, c2x, c1z, w),
                    (c1x, c1y, c1z, w),
                    (c2x, c1y, c1z, w),
                    color1,
                ),
                Triangle::new(
                    (c1x, c2y, c2z, w),
                    (c2x, c2y, c2z, w),
                    (c2x, c1y, c2z, w),
                    color2,
                ),
            ],
        }
    }
    pub fn rotate(mut self, pitch: f32, roll: f32, yaw: f32) -> Self {
        self.triangles[0] = self.triangles[0].clone().rotate(pitch, roll, yaw);
        self.triangles[1] = self.triangles[1].clone().rotate(pitch, roll, yaw);
        self
    }
    pub fn rotate_around(mut self, pitch: f32, roll: f32, yaw: f32, p: (f32, f32, f32)) -> Self {
        self.triangles[0] = self.triangles[0].clone().rotate_around(pitch, roll, yaw, p);
        self.triangles[1] = self.triangles[1].clone().rotate_around(pitch, roll, yaw, p);
        self
    }
    pub fn rotate_around_center(mut self, pitch: f32, roll: f32, yaw: f32) -> Self {
        let topleft = self.triangles[0].vertices[1].position;
        let bottomright = self.triangles[1].vertices[1].position;

        let w = topleft[0] - bottomright[0];
        let h = topleft[1] - bottomright[1];
        let d = topleft[1] - bottomright[1];

        self.rotate_around(pitch, roll, yaw, (w / 2.0, h / 2.0, d / 2.0))
    }
    pub fn into_raw(self) -> (Vec<Vertex>, u32) {
        let cloned = self
            .triangles
            .into_iter()
            .flat_map(|v| v.into_raw().0.into_iter())
            .collect::<Vec<_>>();
        (cloned, 6)
    }
}

#[derive(Clone)]
pub struct Cube {
    faces: Vec<Rectangle>,
}

impl Cube {
    pub fn new(
        c1 @ (c1x, c1y, c1z): (f32, f32, f32),
        c2 @ (c2x, c2y, c2z): (f32, f32, f32),
        w: f32,
        color1: (f32, f32, f32, f32),
        color2: (f32, f32, f32, f32),
    ) -> Self {
        Self {
            faces: vec![
                Rectangle::new((c2x, c1y, c2z), (c1x, c2y, c2z), w, color1, color2),
                Rectangle::new((c1x, c2y, c1z), c2, w, color1, color2),
                Rectangle::new(c1, (c2x, c1y, c2z), w, color1, color2),
                Rectangle::new(c1, (c2x, c2y, c1z), w, color1, color2),
                Rectangle::new((c2x, c1y, c1z), c2, w, color1, color2),
                Rectangle::new(c1, (c1x, c2y, c2z), w, color1, color2),
            ],
        }
    }
    pub fn rotate(mut self, pitch: f32, roll: f32, yaw: f32) -> Self {
        self.faces[0] = self.faces[0].clone().rotate(pitch, roll, yaw);
        self.faces[1] = self.faces[1].clone().rotate(pitch, roll, yaw);
        self.faces[2] = self.faces[2].clone().rotate(pitch, roll, yaw);
        self.faces[3] = self.faces[3].clone().rotate(pitch, roll, yaw);
        self.faces[4] = self.faces[4].clone().rotate(pitch, roll, yaw);
        self.faces[5] = self.faces[5].clone().rotate(pitch, roll, yaw);
        self
    }
    pub fn into_raw(self) -> (Vec<Vertex>, u32) {
        let vertices = self
            .faces
            .clone()
            .into_iter()
            .flat_map(|v| v.into_raw().0.into_iter())
            .collect::<Vec<_>>();
        (vertices, self.faces.len() as u32)
    }
}
