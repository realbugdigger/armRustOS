//! Heap allocation.

use crate::{
    backtrace, bsp, common, debug, info,
    memory::{Address, Virtual},
    synchronization,
    synchronization::IRQSafeNullLock,
    warn,
};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null;
use core::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use linked_list_allocator::Heap as LinkedListHeap;
use spin::Once;


/// A heap allocator that can be lazily initialized.
pub struct HeapAllocator {
    inner: IRQSafeNullLock<BuddyAllocator>,
}


// #[global_allocator]
// lazy_static! {
//     pub static ref KERNEL_HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();
// }

#[global_allocator]
static KERNEL_ALLOC: HeapAllocator = HeapAllocator::new();

static KERNEL_HEAP_ALLOCATOR: Once<HeapAllocator> = Once::new();

#[inline(always)]
fn debug_print_alloc_dealloc(operation: &'static str, ptr: *mut u8, layout: Layout) {
    let size = layout.size();
    let (size_h, size_unit) = common::size_human_readable_ceil(size);
    let addr = Address::<Virtual>::new(ptr as usize);

    debug!(
        "Kernel Heap: {}\n      \
        Size:     {:#x} ({} {})\n      \
        Start:    {}\n      \
        End excl: {}\n\n      \
        {}",
        operation,
        size,
        size_h,
        size_unit,
        addr,
        addr + size,
        backtrace::Backtrace
    );
}


use synchronization::interface::Mutex;
use crate::memory::buddy_alloc::BuddyAllocator;

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}

/// Return a reference to the kernel's heap alloc.
pub fn kernel_heap_allocator() -> &'static HeapAllocator {
    &KERNEL_HEAP_ALLOCATOR.get().unwrap()
}

impl HeapAllocator {
    pub fn new() -> Self {
        Self {
            inner: _,
        }
    }

    /// Create an instance.
    pub fn init() -> Self {
        Self {
            inner: IRQSafeNullLock::new(BuddyAllocator::new()),
        }
    }

    /// Print the current heap usage.
    pub fn print_usage(&self) {
        warn!("Not available yet !!!!")
        // let (used, free) = KERNEL_HEAP_ALLOCATOR
        //     .inner
        //     .lock(|inner| (inner.used(), inner.free()));
        //
        // if used >= 1024 {
        //     let (used_h, used_unit) = common::size_human_readable_ceil(used);
        //     info!("      Used: {} Byte ({} {})", used, used_h, used_unit);
        // } else {
        //     info!("      Used: {} Byte", used);
        // }
        //
        // if free >= 1024 {
        //     let (free_h, free_unit) = common::size_human_readable_ceil(free);
        //     info!("      Free: {} Byte ({} {})", free, free_h, free_unit);
        // } else {
        //     info!("      Free: {} Byte", free);
        // }
    }
}

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let result = KERNEL_HEAP_ALLOCATOR.get().unwrap()
            .inner
            .lock(|inner| inner.alloc(layout).ok());

        match result {
            None => core::ptr::null_mut(),
            Some(allocation) => {
                // let ptr = allocation.as_ptr();
                let ptr = allocation;

                debug_print_alloc_dealloc("Allocation", ptr, layout);

                ptr
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        KERNEL_HEAP_ALLOCATOR.get().unwrap()
            .inner
            .lock(|inner| inner.dealloc(ptr, layout));

        debug_print_alloc_dealloc("Free", ptr, layout);
    }
}

fn init_heap() -> HeapAllocator {
    HeapAllocator {
        inner: IRQSafeNullLock::new(BuddyAllocator::new()),
    }
}

/// Query the BSP for the heap region and initialize the kernel's heap alloc with it.
pub fn kernel_init_heap_allocator() {
    static INIT_DONE: AtomicBool = AtomicBool::new(false);
    if INIT_DONE.load(Ordering::Relaxed) {
        warn!("Already initialized");
        return;
    }

    let region = bsp::memory::mmu::virt_heap_region();

    // KERNEL_HEAP_ALLOCATOR.inner.lock(|inner| unsafe {
    //     inner.init(region.start_addr().as_usize() as *mut u8, region.size())
    // });

    // KERNEL_HEAP_ALLOCATOR.get().unwrap().inner.lock(|inner| unsafe {
    //     inner.init(region.start_addr().as_usize() as *mut u8, region.size())
    // });

    let allocator = KERNEL_HEAP_ALLOCATOR.call_once(|| {
        // let mut heap_allocator = KERNEL_ALLOC;
        KERNEL_ALLOC.init_heap(region.start_addr().as_usize() as *mut u8, region.size());
        KERNEL_ALLOC
    });

    INIT_DONE.store(true, Ordering::Relaxed);
}