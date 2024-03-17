# ARM64 Rust OS

The reason for making this project was to expand my knowledge on OS internals, virtual memory and memory allocators as well as diving into ARM64 architecture.
There were many online resources that were used for knowledge gaining and inspiration but honorable mention goes to [rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials])

***

1. Check if we can replace the way synchronization of static mutabels is done. In project some dummy trait was introduced. Maybe add a spin crate? Check this one out !!!

***

In order to provide a clean abstraction between arch, bsp and generic kernel code, interface traits are provided whenever possible and where it makes sense. They are defined in the respective subsystem module and help to enforce the idiom of program to an interface, not an implementation. For example, there will be a common IRQ handling interface which the two different interrupt controller drivers of both Raspberrys will implement, and only export the interface to the rest of the kernel.

**Maybe remove all `interface`'s ??? Look more into this, possibly there are some thing worth cleaning up**

***

identity translation for the entire physical address space

***

What does `pub` mean in rust? Available outside of the module ???

***


