#[derive(Clone, Copy, Debug)]
pub struct LidarPoint {
    pub dist_mm: u32,
    pub angle_mdeg: i32,
    pub intensity: u8,
    pub ts_ns: u64,
}

pub struct LidarStream {
    pub ring: [LidarPoint; 2048],
    pub head: usize,
    pub tail: usize,
}

impl LidarStream {
    pub const fn new() -> Self {
        Self {
            ring: [LidarPoint {
                dist_mm: 0,
                angle_mdeg: 0,
                intensity: 0,
                ts_ns: 0,
            }; 2048],
            head: 0,
            tail: 0,
        }
    }
    pub fn push(&mut self, p: LidarPoint) {
        self.ring[self.head % 2048] = p;
        self.head = self.head.wrapping_add(1);
    }
    pub fn pop(&mut self) -> Option<LidarPoint> {
        if self.tail == self.head {
            None
        } else {
            let p = self.ring[self.tail % 2048];
            self.tail += 1;
            Some(p)
        }
    }
}

pub trait Lidar: Send + Sync {
    fn init(&mut self) -> Result<(), ()>;
    fn read(&mut self) -> Option<LidarPoint>;
}
