//! BSP Memory Management Unit.

use crate::{
    memory::{
        mmu::{
            self as generic_mmu, AccessPermissions, AddressSpace, AssociatedTranslationTable,
            AttributeFields, MemAttributes, MemoryRegion, PageAddress, TranslationGranule,
        },
        Physical, Virtual,
    },
    synchronization::InitStateLock,
};

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

type KernelTranslationTable = <KernelVirtAddrSpace as AssociatedTranslationTable>::TableStartFromBottom;

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

/// The translation granule chosen by this BSP. This will be used everywhere else in the kernel to
/// derive respective data structures and their sizes. For example, the `crate::memory::mmu::Page`.
pub type KernelGranule = TranslationGranule<{ 64 * 1024 }>;

/// The kernel's virtual address space defined by this BSP.
pub type KernelVirtAddrSpace = AddressSpace<{ 1024 * 1024 * 1024 }>;

//--------------------------------------------------------------------------------------------------
// Global instances
//--------------------------------------------------------------------------------------------------

/// The kernel translation tables.
///
/// It is mandatory that InitStateLock is transparent.
///
/// That is, `size_of(InitStateLock<KernelTranslationTable>) == size_of(KernelTranslationTable)`.
static KERNEL_TABLES: InitStateLock<KernelTranslationTable> = InitStateLock::new(KernelTranslationTable::new());

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

/// Helper function for calculating the number of pages the given parameter spans.
const fn size_to_num_pages(size: usize) -> usize {
    assert!(size > 0);
    assert!(size % KernelGranule::SIZE == 0);

    size >> KernelGranule::SHIFT
}

/// The code pages of the kernel binary.
fn virt_code_region() -> MemoryRegion<Virtual> {
    let num_pages = size_to_num_pages(super::code_size());

    let start_page_addr = super::virt_code_start();
    let end_exclusive_page_addr = start_page_addr.checked_offset(num_pages as isize).unwrap();

    MemoryRegion::new(start_page_addr, end_exclusive_page_addr)
}

/// The data pages of the kernel binary.
fn virt_data_region() -> MemoryRegion<Virtual> {
    let num_pages = size_to_num_pages(super::data_size());

    let start_page_addr = super::virt_data_start();
    let end_exclusive_page_addr = start_page_addr.checked_offset(num_pages as isize).unwrap();

    MemoryRegion::new(start_page_addr, end_exclusive_page_addr)
}

/// The boot core stack pages.
fn virt_boot_core_stack_region() -> MemoryRegion<Virtual> {
    let num_pages = size_to_num_pages(super::boot_core_stack_size());

    let start_page_addr = super::virt_boot_core_stack_start();
    let end_exclusive_page_addr = start_page_addr.checked_offset(num_pages as isize).unwrap();

    MemoryRegion::new(start_page_addr, end_exclusive_page_addr)
}

// The binary is still identity mapped, so use this trivial conversion function for mapping below.

fn kernel_virt_to_phys_region(virt_region: MemoryRegion<Virtual>) -> MemoryRegion<Physical> {
    MemoryRegion::new(
        PageAddress::from(virt_region.start_page_addr().into_inner().as_usize()),
        PageAddress::from(
            virt_region
                .end_exclusive_page_addr()
                .into_inner()
                .as_usize(),
        ),
    )
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Return a reference to the kernel's translation tables.
pub fn kernel_translation_tables() -> &'static InitStateLock<KernelTranslationTable> {
    &KERNEL_TABLES
}

/// The MMIO remap pages.
pub fn virt_mmio_remap_region() -> MemoryRegion<Virtual> {
    let num_pages = size_to_num_pages(super::mmio_remap_size());

    let start_page_addr = super::virt_mmio_remap_start();
    let end_exclusive_page_addr = start_page_addr.checked_offset(num_pages as isize).unwrap();

    MemoryRegion::new(start_page_addr, end_exclusive_page_addr)
}

/// Map the kernel binary.
///
/// # Safety
///
/// - Any miscalculation or attribute error will likely be fatal. Needs careful manual checking.
pub unsafe fn kernel_map_binary() -> Result<(), &'static str> {
    generic_mmu::kernel_map_at(
        "Kernel boot-core stack",
        &virt_boot_core_stack_region(),
        &kernel_virt_to_phys_region(virt_boot_core_stack_region()),
        &AttributeFields {
            mem_attributes: MemAttributes::CacheableDRAM,
            acc_perms: AccessPermissions::ReadWrite,
            execute_never: true,
        },
    )?;

    generic_mmu::kernel_map_at(
        "Kernel code and RO data",
        &virt_code_region(),
        &kernel_virt_to_phys_region(virt_code_region()),
        &AttributeFields {
            mem_attributes: MemAttributes::CacheableDRAM,
            acc_perms: AccessPermissions::ReadOnly,
            execute_never: false,
        },
    )?;

    generic_mmu::kernel_map_at(
        "Kernel data and bss",
        &virt_data_region(),
        &kernel_virt_to_phys_region(virt_data_region()),
        &AttributeFields {
            mem_attributes: MemAttributes::CacheableDRAM,
            acc_perms: AccessPermissions::ReadWrite,
            execute_never: true,
        },
    )?;

    Ok(())
}
