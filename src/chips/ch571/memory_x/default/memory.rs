use crate::mem_layout::{Access, MemoryRegion, MemoryRegionKind, MemoryRole::*, Mode::*};

pub static MEMORY: &[MemoryRegion] = &[
    MemoryRegion {
        name: "CODEFLASH",
        kind: MemoryRegionKind::Flash,
        role: Application,
        address: 0x0,
        size: 196608,
        modes: &[Standard {
            erase_size: 4096,
            write_size: 4,
        }],
        access: None,
    },
    MemoryRegion {
        name: "DATAFLASH",
        kind: MemoryRegionKind::Flash,
        role: Vendor,
        address: 0x70000,
        size: 32768,
        modes: &[Standard {
            erase_size: 256,
            write_size: 1,
        }],
        access: None,
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        role: Ram,
        address: 0x20003800,
        size: 18432,
        modes: &[],
        access: None,
    },
];
