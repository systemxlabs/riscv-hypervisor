use bitmap_allocator::BitAlloc;

use crate::{
    config::{PAGE_SIZE_4K, PHYS_MEMORY_END},
    error::{HypervisorError, HypervisorResult},
    mem::addr::{align_down, align_up, HostPhysAddr},
};
use spin::Mutex;

pub static PHYS_FRAME_ALLOCATOR: Mutex<PhysFrameAllocator> = Mutex::new(PhysFrameAllocator::new());

pub fn init_frame_allocator() {
    extern "C" {
        fn ehypervisor();
    }
    let start = ehypervisor as usize;
    let size = PHYS_MEMORY_END - start;
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

    pub fn alloc_frames(&mut self, num_frames: usize) -> HypervisorResult<HostPhysAddr> {
        match num_frames.cmp(&1) {
            core::cmp::Ordering::Equal => self
                .inner
                .alloc()
                .map(|idx| (idx * PAGE_SIZE_4K + self.base).into()),
            core::cmp::Ordering::Greater => self
                .inner
                .alloc_contiguous(num_frames, 1)
                .map(|idx| (idx * PAGE_SIZE_4K + self.base).into()),
            _ => return Err(HypervisorError::InvalidParam),
        }
        .ok_or(HypervisorError::NoMemory)
        .inspect(|_| self.used_frames += num_frames)
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
