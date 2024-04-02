# ARM64 Rust OS *(project in progress)*

The reason for making this small operating system in the Rust programming language was to expand my knowledge on OS internals, virtual memory and memory allocators as well as diving into ARM64 architecture.

Online resources that were used for knowledge gaining and inspiration:
- https://os.phil-opp.com/
- https://wiki.osdev.org/Main_Page
- https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials (honorable mention)

***

To-do future enhancements:
- Support 4KB and 16KB pages (*in progress*)
- Buddy allocator (*in progress*)
  - Build SLOB allocator on top of it
 
- Add transition from EL1 to EL0 and some userland app
- Implement some memory protection mechanism (PAC, Memory tagging)???
- Add *multitasking* to OS
