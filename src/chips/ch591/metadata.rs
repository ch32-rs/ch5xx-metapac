include!("../metadata_0009.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static NV_STRUCTS: &[NvStructBinding] = &[];
pub static METADATA: Metadata = Metadata {
    name: "CH591",
    family: "QingKe RISC-V-based, BLE MCU",
    line: "RISC-V Core BLE Wireless MCU",
    memory: crate::memory_select::MEMORY,
    nv_structs: NV_STRUCTS,
    memory_options: &[MemoryOption {
        name: "default",
        region_sizes: &[],
    }],
    default_memory_option: "default",
    peripherals: PERIPHERALS,
    // nvic_priority_bits: 0,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
