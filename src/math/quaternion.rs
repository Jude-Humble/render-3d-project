pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w, }
    }

    pub fn as_conjugate(&self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z, w: self.w }
    }

    pub fn to_conjugate(&mut self) {
        self.x *= -1.0;
        self.y *= -1.0;
        self.z *= -1.0;
    }
}
