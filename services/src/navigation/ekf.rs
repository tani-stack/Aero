pub struct Ekf {
    pub s: [f32; 16],
    pub cov: [[f32; 16]; 16],
    pub ok: bool,
}

impl Ekf {
    pub fn new() -> Self {
        Self {
            s: [0.0; 16],
            cov: [[0.0; 16]; 16],
            ok: false,
        }
    }
    pub fn init(&mut self, p: [f32; 3]) {
        self.s[0..3].copy_from_slice(&p);
        self.s[6] = 1.0;
        for i in 0..16 {
            self.cov[i][i] = 0.1;
        }
        self.ok = true;
    }
    pub fn predict(&mut self, a: [f32; 3], _g: [f32; 3], dt: f32) {
        if !self.ok {
            return;
        }
        let gravity = [0.0, 0.0, 9.81];
        for i in 0..3 {
            self.s[3 + i] += (a[i] - gravity[i]) * dt;
            self.s[i] += self.s[3 + i] * dt;
        }
    }
    pub fn update_gps(&mut self, p: [f32; 3], v: [f32; 3]) {
        for i in 0..3 {
            self.s[i] += 0.1 * (p[i] - self.s[i]);
            self.s[3 + i] += 0.1 * (v[i] - self.s[3 + i]);
        }
    }
}
