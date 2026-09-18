/*
Memory configuration for the MPS3-AN536 machine (Cortex-R52).
Reused from rust-embedded/aarch32 examples/mps3-an536/memory.x.
See https://github.com/qemu/qemu/blob/master/hw/arm/mps3r.c
*/
MEMORY {
    QSPI : ORIGIN = 0x08000000, LENGTH = 8M
    DDR  : ORIGIN = 0x20000000, LENGTH = 128M
}
REGION_ALIAS("VECTORS", QSPI);
REGION_ALIAS("CODE", QSPI);
REGION_ALIAS("DATA", DDR);
SECTIONS {
    .irq_entries : ALIGN(4)
    {
        __irq_entries_start = .;
        KEEP(*(.irq_entries));
        . = ALIGN(4);
        __irq_entries_end = .;
    } > CODE
} INSERT AFTER .text;
PROVIDE(_hyp_stack_size = 1M);
PROVIDE(_und_stack_size = 1M);
PROVIDE(_svc_stack_size = 1M);
PROVIDE(_abt_stack_size = 1M);
PROVIDE(_irq_stack_size = 1M);
PROVIDE(_fiq_stack_size = 1M);
