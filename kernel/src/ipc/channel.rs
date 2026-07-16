use spin::Mutex;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Rights: u32 {
        const SEND = 1;
        const RECV = 2;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Capability {
    pub id: u64,
    pub rights: Rights,
    pub gen: u32,
}

pub struct CapTable {
    map: heapless::FnvIndexMap<u64, Capability, 64>,
}

impl CapTable {
    pub fn new() -> Self {
        Self {
            map: heapless::FnvIndexMap::new(),
        }
    }
    pub fn has(&self, id: u64, r: Rights) -> bool {
        self.map
            .get(&id)
            .map(|c| c.rights.contains(r))
            .unwrap_or(false)
    }
    pub fn insert(&mut self, c: Capability) {
        let _ = self.map.insert(c.id, c);
    }
}

pub struct Channel {
    pub cap: Capability,
    ring: Mutex<Ring>,
}

struct Ring {
    buf: [u8; 4096],
    h: usize,
    t: usize,
}

impl Channel {
    pub fn new(cap: Capability) -> Self {
        Self {
            cap,
            ring: Mutex::new(Ring {
                buf: [0; 4096],
                h: 0,
                t: 0,
            }),
        }
    }
    pub fn send(&self, tbl: &CapTable, d: &[u8]) -> Result<(), ()> {
        if !tbl.has(self.cap.id, Rights::SEND) {
            return Err(());
        }
        let mut r = self.ring.lock();
        for &b in d {
            let h = r.h;
            r.buf[h] = b;
            r.h = (h + 1) % 4096;
        }
        Ok(())
    }
    pub fn recv(&self, tbl: &CapTable, out: &mut [u8]) -> Result<usize, ()> {
        if !tbl.has(self.cap.id, Rights::RECV) {
            return Err(());
        }
        let mut r = self.ring.lock();
        let mut n = 0;
        while r.t != r.h && n < out.len() {
            let t = r.t;
            out[n] = r.buf[t];
            r.t = (t + 1) % 4096;
            n += 1;
        }
        Ok(n)
    }
}
