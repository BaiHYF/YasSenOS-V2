use core::sync::atomic::{AtomicU64, Ordering};

use alloc::sync::Arc;
use x86::current;
use x86_64::{
    structures::paging::{mapper::UnmapError, Page},
    VirtAddr,
};

use super::{FrameAllocatorRef, MapperRef};

// user process runtime heap
// 0x100000000 bytes -> 4GiB
// from 0x0000_2000_0000_0000 to 0x0000_2000_ffff_fff8
pub const HEAP_START: u64 = 0x2000_0000_0000;
pub const HEAP_PAGES: u64 = 0x100000;
pub const HEAP_SIZE: u64 = HEAP_PAGES * crate::memory::PAGE_SIZE;
pub const HEAP_END: u64 = HEAP_START + HEAP_SIZE - 8;

/// User process runtime heap
///
/// always page aligned, the range is [base, end)
pub struct Heap {
    /// the base address of the heap
    ///
    /// immutable after initialization
    base: VirtAddr,

    /// the current end address of the heap
    ///
    /// use atomic to allow multiple threads to access the heap
    end: Arc<AtomicU64>,
}

impl Heap {
    pub fn empty() -> Self {
        Self {
            base: VirtAddr::new(HEAP_START),
            end: Arc::new(AtomicU64::new(HEAP_START)),
        }
    }

    pub fn fork(&self) -> Self {
        Self {
            base: self.base,
            end: self.end.clone(),
        }
    }

    pub fn brk(
        &self,
        new_end: Option<VirtAddr>,
        mapper: MapperRef,
        alloc: FrameAllocatorRef,
    ) -> Option<VirtAddr> {

        // FIXME: if new_end is None, return the current end address
        if new_end.is_none() {
            return Some(VirtAddr::new(self.end.load(Ordering::Relaxed)));
        }
        // FIXME: check if the new_end is valid (in range [base, base + HEAP_SIZE])
        let new_end = new_end.unwrap();
        if new_end > self.base + HEAP_SIZE || new_end < self.base {
            error!("Brk: new_end is invalid: {:#x}", new_end);
            return None;
        }
        
        // FIXME: calculate the difference between the current end and the new end
        
        let diff = new_end.as_u64() - self.end.load(Ordering::Acquire);
        let current_end = self.end.load(Ordering::Acquire);

        let mut current_end_page = Page::containing_address(VirtAddr::new(current_end));
        let mut new_end_page = Page::containing_address(new_end);

        if current_end != self.base.as_u64() { current_end_page += 1; }
        if new_end != self.base { new_end_page += 1; }


        // NOTE: print the heap difference for debugging
        debug!("Brk: diff: {:#x}", diff);
        debug!("Brk: current_end: {:#x}", current_end);
        debug!("Brk: new_end: {:#x}", new_end);
        debug!("Brk: current_end_page: {:#x}", current_end_page.start_address().as_u64());
        debug!("Brk: new_end_page: {:#x}", new_end_page.start_address().as_u64());
        // FIXME: do the actual mapping or unmapping

        if diff > 0 {
            // expand heap
            let range = Page::range_inclusive(current_end_page, new_end_page - 1);
            elf::map_range(range, mapper, alloc, true).ok()?;
        }
        else if diff < 0 {
            // shrink heap
            let range = Page::range_inclusive(new_end_page, current_end_page - 1);
            elf::unmap_range(range, mapper, alloc, true).ok()?;
        }

        // FIXME: update the end address

        self.end.store(new_end.as_u64(), Ordering::Release);
        Some(new_end)

    }

    pub(super) fn clean_up(
        &self,
        mapper: MapperRef,
        dealloc: FrameAllocatorRef,
    ) -> Result<(), UnmapError> {
        if self.memory_usage() == 0 {
            return Ok(());
        }

        // FIXME: load the current end address and **reset it to base** (use `swap`)
        let end = self.end.swap(self.base.as_u64(), Ordering::Relaxed);

        let start_page = Page::containing_address(self.base);
        let end_page = Page::containing_address(VirtAddr::new(end));
        let range = Page::range_inclusive(start_page, end_page);

        // FIXME: unmap the heap pages
        elf::unmap_range(range, mapper, dealloc, true)?;

        Ok(())
    }

    pub fn memory_usage(&self) -> u64 {
        self.end.load(Ordering::Relaxed) - self.base.as_u64()
    }
}

impl core::fmt::Debug for Heap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Heap")
            .field("base", &format_args!("{:#x}", self.base.as_u64()))
            .field(
                "end",
                &format_args!("{:#x}", self.end.load(Ordering::Relaxed)),
            )
            .finish()
    }
}
