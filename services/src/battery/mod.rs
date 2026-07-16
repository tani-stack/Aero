pub mod fuel_gauge;

use fuel_gauge::FuelGauge;

pub struct BatteryState {
    pub v: u32,
    pub i: i32,
    pub soc: u8,
    pub temp: i16,
}

pub struct BatteryMgr<G: FuelGauge> {
    g: G,
    state: BatteryState,
    mah_used: f32,
}

impl<G: FuelGauge> BatteryMgr<G> {
    pub fn new(g: G) -> Self {
        Self {
            g,
            state: BatteryState {
                v: 0,
                i: 0,
                soc: 100,
                temp: 25,
            },
            mah_used: 0.0,
        }
    }
    pub fn update(&mut self, dt: f32) {
        let v = self.g.voltage_mv();
        let i = self.g.current_ma();
        self.mah_used += (i as f32) * dt / 3600.0;
        self.state = BatteryState {
            v,
            i,
            soc: self.g.soc(),
            temp: 25,
        };
    }
}
