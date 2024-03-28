# ARM64 Rust OS

The reason for making this project was to expand my knowledge on OS internals, virtual memory and memory allocators as well as diving into ARM64 architecture.
There were many online resources that were used for knowledge gaining and inspiration but honorable mention goes to [rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials]) which code was used for some parts of OS.

***

To-do future enhancements:
- Buddy allocator (*in progress*)
  - Build SLOB allocator on top of it
 
- Add transition from EL1 to EL0 and some userland app
- Implement some memory protection mechanism (PAC, Memory tagging)???
- Add *multitasking* to OS
