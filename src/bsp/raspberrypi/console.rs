//! BSP console facilities.

use crate::console;
use core::{alloc, fmt};

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

/// A mystical, magical device for generating QEMU output out of the void.
struct QEMUOutput;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

/// Implementing `core::fmt::Write` enables usage of the `format_args!` macros, which in turn are
/// used to implement the `kernel`'s `print!` and `println!` macros. By implementing `write_str()`,
/// we get `write_fmt()` automatically.
///
/// See [`src/print.rs`].
///
/// [`src/print.rs`]: ../../print/index.html
impl console::interface::Write for QEMUOutput {

    fn write_char(&self, c: char) {
        // Implement the method write_char to write a character to QEMUOutput
        // You can reuse the code in fmt::Write's write_str method
        unsafe {
            core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
        }
    }

    fn write_str(&self, s: &str) {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }
    }

    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        // In here, you could use write_str from fmt::Write trait to implement the write_fmt method
        // using the format args.
        // self.write_str(&format!("{}", args));

        Ok(())
    }

    fn flush(&self) {
        // Implement the method to flush the buffer if necessary
        // It would be NOP for QEMUOutput, hence left empty
    }
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

// pub fn format(args: fmt::Arguments) -> alloc::string::String {
//     use alloc::string::ToString;
//     let mut output = String::new();
//     let _ = fmt::write(&mut output, args);
//     output
// }

/// Return a reference to the console.
pub fn console() -> impl console::interface::Write {
    QEMUOutput {}
}