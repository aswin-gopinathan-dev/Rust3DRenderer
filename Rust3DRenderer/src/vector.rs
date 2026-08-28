pub enum Axis {
    XAxis,
    YAxis,
    ZAxis,
}

#[derive(Clone, Copy, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn rotate(&mut self, axis: Axis, angle: f32) {
        match axis {
            Axis::XAxis => {
                let result = Vec3 {
                    x: self.x,
                    y: self.y * angle.cos() - self.z * angle.sin(),
                    z: self.y * angle.sin() + self.z * angle.cos(),
                };
                *self = result;
            }
            Axis::YAxis => {
                let result = Vec3 {
                    x: self.x * angle.cos() - self.z * angle.sin(),
                    y: self.y,
                    z: self.x * angle.sin() + self.z * angle.cos(),
                };
                *self = result;
            }
            Axis::ZAxis => {
                let result = Vec3 {
                    x: self.x * angle.cos() - self.y * angle.sin(),
                    y: self.x * angle.sin() + self.y * angle.cos(),
                    z: self.z,
                };
                *self = result;
            }
        }
    }
}
