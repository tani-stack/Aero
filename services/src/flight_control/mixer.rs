pub enum Frame {
    QuadX,
    HexaX,
}

pub struct MotorMixer {
    frame: Frame,
}

impl MotorMixer {
    pub fn new(f: Frame) -> Self {
        Self { frame: f }
    }
    pub fn mix(&self, thr: f32, roll: f32, pitch: f32, yaw: f32) -> [u32; 8] {
        let mut out = [0u32; 8];
        match self.frame {
            Frame::QuadX => {
                let m = [
                    thr + roll + pitch - yaw,
                    thr - roll + pitch + yaw,
                    thr - roll - pitch - yaw,
                    thr + roll - pitch + yaw,
                ];
                for i in 0..4 {
                    let mut val = m[i];
                    if val > 1.0 {
                        val = 1.0;
                    } else if val < 0.0 {
                        val = 0.0;
                    }
                    out[i] = (val * 1000.0) as u32 + 1000;
                }
            }
            Frame::HexaX => {
                for i in 0..6 {
                    out[i] = 1500;
                }
            }
        }
        out
    }
    pub fn saturate(m: &mut [u32; 8]) {
        for v in m.iter_mut() {
            if *v > 2000 {
                *v = 2000;
            } else if *v < 1000 {
                *v = 1000;
            }
        }
    }
}
