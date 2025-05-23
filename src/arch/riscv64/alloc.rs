use core::sync::atomic::Ordering;

use core::sync::atomic::AtomicUsize;

const PAGE_SIZE: usize = 4096;

pub struct BumpAllocator {
    base: AtomicUsize,
}

impl BumpAllocator {
    pub fn allocate_new_page(&self) -> usize {
        self.base
            .update(Ordering::SeqCst, Ordering::SeqCst, |base| base + PAGE_SIZE)
    }
}

#[inline(never)]
pub fn setup() -> BumpAllocator {
    BumpAllocator {
        base: AtomicUsize::new(0x80C00000),
    }
}
