pub struct CameraFrame { pub id:u64, pub w:u32, pub h:u32, pub ts_ns:u64 }
pub trait Camera: Send+Sync { fn start(&mut self)->Result<(),()>; fn next_frame(&mut self)->Option<CameraFrame>; }
