//! Translation table.

#[path = "../../aarch64/memory/mmu/translation_table.rs"]
mod arch_translation_table;

//--------------------------------------------------------------------------------------------------
// Architectural Public Reexports
//--------------------------------------------------------------------------------------------------
pub use arch_translation_table::KernelTranslationTable;