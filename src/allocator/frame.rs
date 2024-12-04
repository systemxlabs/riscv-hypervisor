use bitmap_allocator::BitAlloc;

use crate::{
    config::PAGE_SIZE_4K,
    dtb::MachineMeta,
    error::{HypervisorError, HypervisorResult},
    mem::addr::{align_down, align_up, HostPhysAddr},
};
use spin::Mutex;

pub static PHYS_FRAME_ALLOCATOR: Mutex<PhysFrameAllocator> = Mutex::new(PhysFrameAllocator::new());

pub fn init_frame_allocator(meta: &MachineMeta) {
    extern "C" {
        fn ehypervisor();
    }
    let start = ehypervisor as usize;
    let phys_mem_end = meta.phys_memory_offset + meta.phys_memory_size;
    let size = phys_mem_end - start;
    PHYS_FRAME_ALLOCATOR.lock().init(start.into(), size);
}

pub struct PhysFrameAllocator {
    base: usize,
    total_frames: usize,
    used_frames: usize,
    inner: bitmap_allocator::BitAlloc1M,
}

impl PhysFrameAllocator {
    pub const fn new() -> Self {
        Self {
            base: 0,
            total_frames: 0,
            used_frames: 0,
            inner: bitmap_allocator::BitAlloc1M::DEFAULT,
        }
    }

    pub fn init(&mut self, start: HostPhysAddr, size: usize) {
        let start = align_up(start.as_usize(), PAGE_SIZE_4K);
        let end = align_down(start + size, PAGE_SIZE_4K);
        self.base = start;
        self.total_frames = (end - start) / PAGE_SIZE_4K;
        self.inner.insert(0..self.total_frames);
    }

    pub fn alloc_frames(
        &mut self,
        num_frames: usize,
        align: usize,
    ) -> HypervisorResult<HostPhysAddr> {
        assert_eq!(align % PAGE_SIZE_4K, 0);
        if num_frames < 1 {
            return Err(HypervisorError::InvalidParam);
        }
        let paddr: Option<HostPhysAddr> = if num_frames == 1 {
            self.inner
                .alloc()
                .map(|idx| (idx * PAGE_SIZE_4K + self.base).into())
        } else {
            self.inner
                .alloc_contiguous(num_frames, align / PAGE_SIZE_4K)
                .map(|idx| (idx * PAGE_SIZE_4K + self.base).into())
        };
        paddr.ok_or(HypervisorError::NoMemory).inspect(|_| {
            self.used_frames += num_frames;
        })
    }

    pub fn dealloc_frames(&mut self, pos: HostPhysAddr, num_frames: usize) {
        // TODO: not decrease `used_frames` if deallocation failed
        self.used_frames -= num_frames;
        self.inner
            .dealloc((pos.as_usize() - self.base) / PAGE_SIZE_4K)
    }

    pub fn total_frames(&self) -> usize {
        self.total_frames
    }

    pub fn used_frames(&self) -> usize {
        self.used_frames
    }

    pub fn available_frames(&self) -> usize {
        self.total_frames - self.used_frames
    }
}
