/* Minimal linker script for aarch64-unknown-none on QEMU virt.
 * Reuse pattern from google/aarch64-rt (image ORIGIN + LENGTH) and
 * rust-embedded/aarch64-cpu examples. QEMU virt loads -kernel at 0x40080000.
 */
MEMORY {
    image : ORIGIN = 0x40080000, LENGTH = 2M
}
SECTIONS {
    . = ORIGIN(image);
    .text : { *(.text._start) *(.text*) *(.rodata*) *(.rodata.*) *(.srodata*) } >image
    .data : { *(.data*) *(.sdata*) *(.sdata.*) } >image
    .bss : { *(.bss*) *(.sbss*) *(.bss.*) *(COMMON) } >image
    .ARM.attributes 0 : { *(.ARM.attributes) }
    /DISCARD/ : { *(.note*) *(.comment*) *(.eh_frame*) *(.debug*) }
}
ENTRY(_start)
PROVIDE(__stack_top = ORIGIN(image) + LENGTH(image));
