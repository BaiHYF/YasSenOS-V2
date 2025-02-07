#![no_std]
pub use uefi::data_types::chars::*;
pub use uefi::data_types::*;
pub use uefi::prelude::SystemTable;
pub use uefi::proto::console::gop::{GraphicsOutput, ModeInfo};
pub use uefi::table::boot::{MemoryAttribute, MemoryDescriptor, MemoryType};
pub use uefi::table::runtime::*;
pub use uefi::table::Runtime;
pub use uefi::Status as UefiStatus;

use arrayvec::{ArrayString, ArrayVec};
use x86_64::structures::paging::page::PageRangeInclusive;
use xmas_elf::ElfFile;

pub mod allocator;
pub mod config;
pub mod fs;

#[macro_use]
extern crate log;

pub type MemoryMap = ArrayVec<MemoryDescriptor, 256>;
pub type KernelPages = ArrayVec<PageRangeInclusive, 8>;
pub type AppListRef = Option<&'static ArrayVec<App<'static>, 16>>;

/// This structure represents the information that the bootloader passes to the kernel.
pub struct BootInfo {
    /// The memory map
    pub memory_map: MemoryMap,

    /// The offset into the virtual address space where the physical memory is mapped.
    pub physical_memory_offset: u64,

    /// UEFI SystemTable
    pub system_table: SystemTable<Runtime>,

    // Loaded apps
    pub loaded_apps: Option<ArrayVec<App<'static>, 16>>,

    // Log Level
    pub log_level: &'static str,

    // Kernel pages
    pub kernel_pages: KernelPages,    
}

/// App information
pub struct App<'a> {
    /// The name of app
    pub name: ArrayString<16>,
    /// The ELF file
    pub elf: ElfFile<'a>,
}

/// This is copied from https://docs.rs/bootloader/0.10.12/src/bootloader/lib.rs.html
/// Defines the entry point function.
///
/// The function must have the signature `fn(&'static BootInfo) -> !`.
///
/// This macro just creates a function named `_start`, which the linker will use as the entry
/// point. The advantage of using this macro instead of providing an own `_start` function is
/// that the macro ensures that the function and argument types are correct.
#[macro_export]
macro_rules! entry_point {
    ($path:path) => {
        #[export_name = "_start"]
        pub extern "C" fn __impl_start(boot_info: &'static $crate::BootInfo) -> ! {
            // validate the signature of the program entry point
            let f: fn(&'static $crate::BootInfo) -> ! = $path;

            f(boot_info)
        }
    };
}
