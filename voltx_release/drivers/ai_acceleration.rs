// VoltX-64 AI Acceleration Driver
const AI_ACCELERATOR_BASE: usize = 0x10000000;

pub struct AiAcceleration {
    base_addr: *mut u32,
}

impl AiAcceleration {
    pub const fn new() -> Self {
        Self {
            base_addr: AI_ACCELERATOR_BASE as *mut u32,
        }
    }

    pub unsafe fn start_processing(&self, command: u32) {
        self.base_addr.write_volatile(command);
    }
}
