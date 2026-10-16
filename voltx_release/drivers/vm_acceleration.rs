// VoltX-64 Virtual Memory & MMU Acceleration Driver
const VM_ACCELERATOR_BASE: usize = 0x10010000;

pub struct VmAcceleration {
    base_addr: *mut u32,
}

impl VmAcceleration {
    pub const fn new() -> Self {
        Self {
            base_addr: VM_ACCELERATOR_BASE as *mut u32,
        }
    }

    pub unsafe fn configure_sv39_table(&self, root_page_table_pa: usize) {
        self.base_addr.write_volatile(root_page_table_pa as u32);
    }
}
