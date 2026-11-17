use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Flashrom",
            extends: None,
            description: Some(
                "Page-latch flash controller.",
            ),
            items: &[
                BlockItem {
                    name: "prepg_ctrl",
                    description: Some(
                        "RW, pre-program mode, auto-erase and start.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PrepgCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "buf_ctrl",
                    description: Some(
                        "RW, latch clear and load control.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "BufCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "erase_ctrl",
                    description: Some(
                        "RW, erase mode and start.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EraseCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "program_ctrl",
                    description: Some(
                        "RW, page program start.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ProgramCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rd_ctrl",
                    description: Some(
                        "RW, read timing; SDK `R8_FLASH_RD_CTRL`, not in the datasheet table.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "lp_ctrl",
                    description: Some(
                        "RW, low-power control; SDK-only, no known consumer.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "prefetch_ctrl",
                    description: Some(
                        "RW, prefetch control; SDK-only, no known consumer.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "read_ctrl",
                    description: Some(
                        "RW, data direction, read start, clock and address auto-increment.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ReadCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rdprctdis_ctrl",
                    description: Some(
                        "WO, read-protect disable key; reads back the handshake state.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "RdprctdisCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chip_status",
                    description: Some(
                        "R, read-protect, debug and halted state.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ChipStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addr",
                    description: Some(
                        "RW, flash address; auto-increments with latch data.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "data",
                    description: Some(
                        "RW, 32-bit latch data; read data when read enable is set.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "data1",
                    description: Some(
                        "R, second word of an aligned read.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "data2",
                    description: Some(
                        "R, third word of an aligned read.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "data3",
                    description: Some(
                        "R, fourth word of an aligned read.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "status",
                    description: Some(
                        "R, controller status.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Status",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "BufCtrl",
            extends: None,
            description: Some(
                "Latch clear and load control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clear",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "load_start",
                    description: Some(
                        "Start a manual latch load.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "load_auto",
                    description: Some(
                        "Latch each DATA write automatically.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ChipStatus",
            extends: None,
            description: Some(
                "Chip flash state.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rd_protect",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_mode",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flash_halted",
                    description: Some(
                        "Flash access blocked by the debugger.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EraseCtrl",
            extends: None,
            description: Some(
                "Erase mode and start.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "0 = page, 1 = sector, 2/3 = bank.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "start",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PrepgCtrl",
            extends: None,
            description: Some(
                "Pre-program mode and start.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "auto_en",
                    description: Some(
                        "Run the configured erase after pre-programming.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mode",
                    description: Some(
                        "0 = page, 1 = sector, 2 = bank with ADDR bit 8 selecting the page, 3 = bank.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "start",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ProgramCtrl",
            extends: None,
            description: Some(
                "Page program start.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "start",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RdprctdisCtrl",
            extends: None,
            description: Some(
                "Read-protect disable handshake.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "disable_flag",
                    description: Some(
                        "Set while the disable sequence is running.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "disable_state",
                    description: Some(
                        "0 = idle, 1-3 = running, 4 = done, 5 = failed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "disable_start",
                    description: Some(
                        "11b when 0x73 can start or 0x99 can clear the disable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ReadCtrl",
            extends: None,
            description: Some(
                "Data direction, read start, clock and address auto-increment.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "cfg_rd_en",
                    description: Some(
                        "1 = DATA reads, 0 = DATA writes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rd_start",
                    description: Some(
                        "Start a read; clears when the read finishes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "auto_increment",
                    description: Some(
                        "Advance ADDR by four words after each transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clk",
                    description: Some(
                        "0 = one clock, 3 = two clocks high.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Status",
            extends: None,
            description: Some(
                "Controller status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ready",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busy",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
