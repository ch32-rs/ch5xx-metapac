use crate::mem_layout::{Access, MemoryRegion, MemoryRegionKind, MemoryRole::*, Mode::*};

pub static MEMORY: &[MemoryRegion] = &[
    MemoryRegion {
        name: "CODEFLASH",
        kind: MemoryRegionKind::Flash,
        role: Application,
        address: 0x0,
        size: 245760,
        modes: &[
            Standard {
                erase_size: 256,
                write_size: 256,
            },
        ],
        access: None,
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        role: Ram,
        address: 0x20000000,
        size: 32768,
        modes: &[],
        access: None,
    },
];
