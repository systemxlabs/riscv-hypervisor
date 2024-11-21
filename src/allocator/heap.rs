use core::alloc::{GlobalAlloc, Layout};

use buddy_system_allocator::{Heap, LockedHeap};
use log::debug;

use crate::{allocator::frame::PHYS_FRAME_ALLOCATOR, config::PAGE_SIZE_4K};

#[global_allocator]
static HEAP_ALLOCATOR: BuddyHeapAllocator = BuddyHeapAllocator::new();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

pub fn init_heap_allocator() {
    let num_pages = 8;
    let heap_ptr = PHYS_FRAME_ALLOCATOR
        .lock()
        .alloc_frames(num_pages)
        .expect("Free memory should be enough");
    HEAP_ALLOCATOR.init(heap_ptr.as_usize(), num_pages * PAGE_SIZE_4K);
}

pub struct BuddyHeapAllocator {
    inner: LockedHeap<32>,
}

impl BuddyHeapAllocator {
    pub const fn new() -> Self {
        Self {
            inner: LockedHeap::<32>::new(),
        }
    }

    pub fn init(&self, start: usize, size: usize) {
        unsafe { self.inner.lock().init(start, size) };
    }

    pub fn add_memory(&self, start: usize, size: usize) {
        unsafe { self.inner.lock().add_to_heap(start, start + size) };
    }
}

unsafe impl GlobalAlloc for BuddyHeapAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        loop {
            let mut inner = self.inner.lock();
            if let Ok(ptr) = inner.alloc(layout) {
                return ptr.as_ptr();
            } else {
                debug!("starting to expand heap memory");
                let old_size = inner.stats_total_bytes();
                let expand_size = old_size
                    .max(layout.size())
                    .next_power_of_two()
                    .max(PAGE_SIZE_4K);
                // avoid dead lock
                drop(inner);
                if let Ok(heap_ptr) = PHYS_FRAME_ALLOCATOR
                    .lock()
                    .alloc_frames(expand_size / PAGE_SIZE_4K)
                {
                    let heap_ptr = heap_ptr.as_usize();
                    debug!(
                        "expand heap memory: [{:#x}, {:#x})",
                        heap_ptr,
                        heap_ptr + expand_size
                    );
                    self.add_memory(heap_ptr, expand_size);
                } else {
                    return core::ptr::null_mut();
                }
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.inner
            .lock()
            .dealloc(unsafe { core::ptr::NonNull::new_unchecked(ptr) }, layout)
    }
}
