use arrayvec::ArrayVec;
use fdt::Fdt;
use log::{debug, info};

#[derive(Clone, Debug)]
pub struct Device {
    pub base_address: usize,
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct Hart {
    pub hartid: u64,
    pub plic_context: u64,
}

#[derive(Debug, Clone, Default)]
pub struct MachineMeta {
    pub phys_memory_offset: usize,
    pub phys_memory_size: usize,
    pub virtio: ArrayVec<Device, 16>,
}

impl MachineMeta {
    pub fn parse(dtb: usize) -> Self {
        let fdt = unsafe { Fdt::from_ptr(dtb as *const u8) }.unwrap();
        info!("ftd: {:?}", fdt);
        let mut meta = MachineMeta::default();
        for region in fdt.memory().regions() {
            meta.phys_memory_offset = region.starting_address as usize;
            meta.phys_memory_size = region.size.unwrap();
        }
        for node in fdt.find_all_nodes("/soc/virtio_mmio") {
            if let Some(reg) = node.reg().and_then(|mut reg| reg.next()) {
                let paddr = reg.starting_address as usize;
                let size = reg.size.unwrap();
                meta.virtio.push(Device {
                    base_address: paddr,
                    size,
                })
            }
        }
        meta
    }
}

pub fn parse_dtb(dtb: usize) {
    let fdt = unsafe { Fdt::from_ptr(dtb as *const u8) }.unwrap();
    info!("ftd: {:?}", fdt);
}
