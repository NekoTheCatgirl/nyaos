// We are now implementing a Fixed size block allocator!
// See <https://os.phil-opp.com/allocator-designs/#implementation-2> for the reference

mod fixed_block_allocator;

use fixed_block_allocator::FixedSizeBlockAllocator;
use multiboot2::BootInformation;

pub const PAGE_SIZE: usize = 4096;

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

pub fn init_heap(_info: &BootInformation) {
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
}

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB