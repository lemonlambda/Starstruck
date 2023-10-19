use std::ops::{Add, Mul, Div, Sub};

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}
impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }
    }
    pub fn rotate_around(mut self, pitch: f32, roll: f32, yaw: f32, p@(x, y, z): (f32, f32, f32)) -> Self {
        let (ox, oy, oz) = (-x, -y, -z);

        self = self.rotate(pitch, roll, yaw);

        self.position[0] = self.position[0] - ox;
        self.position[1] = self.position[1] - oy;
        self.position[2] = self.position[2] - oz;

        self
    }
    
    pub fn rotate(mut self, pitch: f32, roll: f32, yaw: f32) -> Self {
        let cosa = yaw.to_radians().cos();
        let sina = yaw.to_radians().sin();

        let cosb = pitch.to_radians().cos();
        let sinb = pitch.to_radians().sin();

        let cosc = roll.to_radians().cos();
        let sinc = roll.to_radians().sin();

        let axx = cosa * cosb;
        let axy = cosa * sinb * sinc - sina * cosc;
        let axz = cosa * sinb * cosc + sina * sinc;

        let ayx = sina * cosb; 
        let ayy = sina * sinb * sinc + cosa * cosc;
        let ayz = sina * sinb * cosc - cosa * sinc;

        let azx = -sinb ;
        let azy = cosb * sinc;
        let azz = cosb * cosc;

        let px = self.position[0];
        let py = self.position[1];
        let pz = self.position[2];

        self.position[0] = axx * px + axy * py + axz * pz;
        self.position[1] = ayx * px + ayy * py + ayz * pz;
        self.position[2] = azx * px + azy * py + azz * pz;

        self
    }
}

impl Add<f32> for Vertex {
    type Output = Vertex;
    
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            position: [self.position[0] + rhs, self.position[1] + rhs, self.position[2] + rhs, self.position[3]],
            color: self.color
        }
    }
}
impl Sub<f32> for Vertex {
    type Output = Vertex;
    
    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            position: [self.position[0] - rhs, self.position[1] - rhs, self.position[2] - rhs, self.position[3]],
            color: self.color
        }
    }
}
impl Mul<f32> for Vertex {
    type Output = Vertex;
    
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            position: [self.position[0] * rhs, self.position[1] * rhs, self.position[2] * rhs, self.position[3]],
            color: self.color
        }
    }
}
impl Div<f32> for Vertex {
    type Output = Vertex;
    
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            position: [self.position[0] / rhs, self.position[1] / rhs, self.position[2] / rhs, self.position[3]],
            color: self.color
        }
    }
}
