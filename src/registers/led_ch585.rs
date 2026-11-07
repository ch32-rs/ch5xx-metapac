use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Led",
            extends: None,
            description: Some(
                "LED register.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl_mod",
                    description: Some(
                        "RW, LED mode configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "CtrlMod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clock_div",
                    description: Some(
                        "RW, LED serial clock divider register.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ClockDiv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl_mod1",
                    description: Some(
                        "RW, LED mode configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "CtrlMod1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "status",
                    description: Some(
                        "RW1, LED status register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Status",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifo",
                    description: Some(
                        "WO, LED data FIFO register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Fifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_len",
                    description: Some(
                        "WO, LED DMA transmission length.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 16,
                            fieldset: Some(
                                "DmaLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_cnt",
                    description: Some(
                        "RO, LED DMA remaining count register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "DmaCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_beg",
                    description: Some(
                        "RW, LED DMA starting address.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaBeg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_cur",
                    description: Some(
                        "RO, LED DMA current address.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaCur",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ClockDiv",
            extends: None,
            description: Some(
                "RW, LED serial clock divider register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clock_div",
                    description: Some(
                        "LED output clock frequency division coefficient.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CtrlMod",
            extends: None,
            description: Some(
                "RW, LED mode configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "bit_order",
                    description: Some(
                        "Selection of LED serial data bit sequence.",
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
                    name: "all_clear",
                    description: Some(
                        "LED FIFO/counter/interrupt flag reset to zero.",
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
                    name: "out_polar",
                    description: Some(
                        "LED data output polarity control bit.",
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
                    name: "out_en",
                    description: Some(
                        "LED signal output enable.",
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
                Field {
                    name: "dma_en",
                    description: Some(
                        "LED DMA function and DMA interrupt enable.",
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
                    name: "ie_fifo",
                    description: Some(
                        "FIFO count over half interrupt enabled.",
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
                Field {
                    name: "chan_mod",
                    description: Some(
                        "LED channel mode selection.",
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
            name: "CtrlMod1",
            extends: None,
            description: Some(
                "RW, LED mode configuration register 1.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_loop",
                    description: Some(
                        "Enable LED DMA data cycling usage.",
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
                    name: "ie_send_end",
                    description: Some(
                        "DMA All Data Transfer End Interrupt Enable.",
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
            ],
        },
        FieldSet {
            name: "DmaBeg",
            extends: None,
            description: Some(
                "RW, LED DMA starting address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_beg",
                    description: Some(
                        "DMA starting address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaCnt",
            extends: None,
            description: Some(
                "RO, LED DMA remaining count register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dma_cnt",
                    description: Some(
                        "LED_SMA_maIN Main Buffer Current DMA Remaining Words (16 bits) Count.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaCur",
            extends: None,
            description: Some(
                "RO, LED DMA current address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_cur",
                    description: Some(
                        "LED DMA current address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaLen",
            extends: None,
            description: Some(
                "WO, LED DMA transmission length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dma_length",
                    description: Some(
                        "DMA transmission count length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fifo",
            extends: None,
            description: Some(
                "WO, LED data FIFO register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fifo",
                    description: Some(
                        "LED data FIFO input, 32-bit write.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Status",
            extends: None,
            description: Some(
                "RW1, LED status register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "fifo_count",
                    description: Some(
                        "The byte count value in the current FIFO must be even.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clock",
                    description: Some(
                        "Current LED clock signal level status.",
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
                Field {
                    name: "if_fifo",
                    description: Some(
                        "FIFO count over half interrupt flag, write 1 to reset or write R16_SED_LFO to reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "load_fail",
                    description: Some(
                        "Loading data results.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "if_dma_end",
                    description: Some(
                        "DMA completion flag, write 1 to reset or write R16_SED-DMA_CNT to reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "if_dma_int",
                    description: Some(
                        "DMA transfer end interrupt flag bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
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
