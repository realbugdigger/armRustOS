//--------------------------------------------------------------------------------------------------
// Definitions
//--------------------------------------------------------------------------------------------------

// Load the address of a symbol into a register, PC-relative.
//
// The symbol must lie within +/- 4 GiB of the Program Counter.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add	\register, \register, #:lo12:\symbol
.endm

// Load the address of a symbol into a register, absolute.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_ABS register, symbol
	movz	\register, #:abs_g3:\symbol
	movk	\register, #:abs_g2_nc:\symbol
	movk	\register, #:abs_g1_nc:\symbol
	movk	\register, #:abs_g0_nc:\symbol
.endm

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
.section .text._start

//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:
	// Only proceed if the core executes in EL2. Park it otherwise.
	mrs	x0, CurrentEL
	cmp	x0, {CONST_CURRENTEL_EL2}
	b.ne	.L_parking_loop

	// Only proceed on the boot core. Park it otherwise.
	mrs	x1, MPIDR_EL1
	and	x1, x1, {CONST_CORE_ID_MASK}
	ldr	x2, BOOT_CORE_ID      // provided by bsp/raspberrypi/cpu.rs
	cmp	x1, x2
	b.ne	.L_parking_loop

	// If execution reaches here, it is the boot core.

	// Initialize DRAM.
	ADR_REL	x0, __bss_start
	ADR_REL x1, __bss_end_exclusive

.L_bss_init_loop:
	cmp	x0, x1
	b.eq	.L_prepare_rust
	stp	xzr, xzr, [x0], #16
	b	.L_bss_init_loop

	// Prepare the jump to Rust code.
.L_prepare_rust:
	// Load the base address of the kernel's translation tables.
	ldr	x0, PHYS_KERNEL_TABLES_BASE_ADDR // provided by bsp/raspberrypi/memory/mmu.rs

	// Load the _absolute_ addresses of the following symbols. Since the kernel is linked at
	// the top of the 64 bit address space, these are effectively virtual addresses.
	ADR_ABS	x1, __boot_core_stack_end_exclusive
	ADR_ABS	x2, kernel_init

	// Load the PC-relative address of the stack and set the stack pointer.
	//
	// Since _start() is the first function that runs after the firmware has loaded the kernel
	// into memory, retrieving this symbol PC-relative returns the "physical" address.
	//
	// Setting the stack pointer to this value ensures that anything that still runs in EL2,
	// until the kernel returns to EL1 with the MMU enabled, works as well. After the return to
	// EL1, the virtual address of the stack retrieved above will be used.
	ADR_REL	x3, __boot_core_stack_end_exclusive
	mov	sp, x3

	// Read the CPU's timer counter frequency and store it in ARCH_TIMER_COUNTER_FREQUENCY.
	// Abort if the frequency read back as 0.
	ADR_REL	x4, ARCH_TIMER_COUNTER_FREQUENCY // provided by aarch64/time.rs
	mrs	x5, CNTFRQ_EL0
	cmp	x5, xzr
	b.eq	.L_parking_loop
	str	w5, [x4]

	// Jump to Rust code. x0, x1 and x2 hold the function arguments provided to _start_rust().
	b	_start_rust

	// Infinitely wait for events (aka "park the core").
.L_parking_loop:
	wfe
	b	.L_parking_loop

.size	_start, . - _start
.type	_start, function
.global	_start