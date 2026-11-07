
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "SpiFull",
            extends: None,
            description: Some(
                "SPI_FULL register.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl_mod",
                    description: Some(
                        "SPI_FULL mode control.",
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
                    name: "ctrl_cfg",
                    description: Some(
                        "SPI_FULL configuration control.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "CtrlCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "inter_en",
                    description: Some(
                        "SPI_FULL interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "InterEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clock_div",
                    description: Some(
                        "SPI_FULL master clock divisor.",
                    ),
                    array: None,
                    byte_offset: 0x3,
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
                    name: "slave_pre",
                    description: Some(
                        "SPI_FULL slave preset value.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SlavePre",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "buffer",
                    description: Some(
                        "SPI_FULL data buffer.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Buffer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "run_flag",
                    description: Some(
                        "SPI_FULL work flag.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "RunFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_flag",
                    description: Some(
                        "SPI_FULL interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifo_count",
                    description: Some(
                        "SPI_FULL FIFO count status.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "FifoCount",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "total_cnt",
                    description: Some(
                        "SPI_FULL total byte count, only low 12 bit.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "TotalCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifo",
                    description: Some(
                        "SPI_FULL FIFO register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Fifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifo_count1",
                    description: Some(
                        "SPI_FULL FIFO count status.",
                    ),
                    array: None,
                    byte_offset: 0x13,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "FifoCount1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_now",
                    description: Some(
                        "SPI_FULL DMA current address.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaNow",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_beg",
                    description: Some(
                        "SPI_FULL DMA begin address.",
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
                    name: "dma_end",
                    description: Some(
                        "SPI_FULL DMA end address.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaEnd",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Buffer",
            extends: None,
            description: Some(
                "SPI_FULL data buffer.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "buffer",
                    description: Some(
                        "SPI data buffer.",
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
            name: "ClockDiv",
            extends: None,
            description: Some(
                "SPI_FULL master clock divisor.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clock_div",
                    description: Some(
                        "master clock divisor.",
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
            name: "CtrlCfg",
            extends: None,
            description: Some(
                "SPI_FULL configuration control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_enable",
                    description: Some(
                        "SPI DMA enable.",
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
                    name: "dma_loop",
                    description: Some(
                        "SPI DMA address loop enable.",
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
                    name: "auto_if",
                    description: Some(
                        "enable buffer or FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag.",
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
                    name: "bit_order",
                    description: Some(
                        "SPI bit data order.",
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
            name: "CtrlMod",
            extends: None,
            description: Some(
                "SPI_FULL mode control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mode_slave",
                    description: Some(
                        "SPI slave mode.",
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
                        "force clear SPI FIFO and count.",
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
                    name: "_2wire_mod",
                    description: Some(
                        "SPI enable 2 wire mode.",
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
                    name: "mst_sck_mod_rb_slv_cmd_mod",
                    description: Some(
                        "SPI master clock mode _SPI slave command mode.",
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
                    name: "fifo_dir",
                    description: Some(
                        "SPI FIFO direction.",
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
                    name: "sck_oe",
                    description: Some(
                        "SPI SCK output enable.",
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
                    name: "mosi_oe",
                    description: Some(
                        "SPI MOSI output enable.",
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
                    name: "miso_oe",
                    description: Some(
                        "SPI MISO output enable.",
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
            ],
        },
        FieldSet {
            name: "DmaBeg",
            extends: None,
            description: Some(
                "SPI_FULL DMA begin address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_beg",
                    description: Some(
                        "SPI DMA begin address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaEnd",
            extends: None,
            description: Some(
                "SPI_FULL DMA end address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_end",
                    description: Some(
                        "SPI DMA end address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaNow",
            extends: None,
            description: Some(
                "SPI_FULL DMA current address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_now",
                    description: Some(
                        "SPI DMA current address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fifo",
            extends: None,
            description: Some(
                "SPI_FULL FIFO register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "fifo",
                    description: Some(
                        "SPI FIFO register.",
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
            name: "FifoCount",
            extends: None,
            description: Some(
                "SPI_FULL FIFO count status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "fifo_count",
                    description: Some(
                        "SPI FIFO count status.",
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
            name: "FifoCount1",
            extends: None,
            description: Some(
                "SPI_FULL FIFO count status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "fifo_count1",
                    description: Some(
                        "SPI FIFO count statu.",
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
            name: "IntFlag",
            extends: None,
            description: Some(
                "SPI_FULL interrupt flag.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "if_cnt_end",
                    description: Some(
                        "interrupt flag for SPI total byte count end.",
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
                    name: "if_byte_end",
                    description: Some(
                        "interrupt flag for SPI byte exchanged.",
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
                    name: "if_fifo_hf",
                    description: Some(
                        "interrupt flag for SPI FIFO half.",
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
                    name: "if_dma_end",
                    description: Some(
                        "interrupt flag for SPI DMA completion.",
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
                    name: "if_fifo_ov",
                    description: Some(
                        "interrupt flag for SPI FIFO overflow.",
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
                    name: "free",
                    description: Some(
                        "current SPI free status.",
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
                    name: "if_fst_byte",
                    description: Some(
                        "interrupt flag for SPI slave mode first byte received.",
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
            ],
        },
        FieldSet {
            name: "InterEn",
            extends: None,
            description: Some(
                "SPI_FULL interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ie_cnt_end",
                    description: Some(
                        "enable interrupt for SPI total byte count end.",
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
                    name: "ie_byte_end",
                    description: Some(
                        "enable interrupt for SPI byte exchanged.",
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
                    name: "ie_fifo_hf",
                    description: Some(
                        "enable interrupt for SPI FIFO half.",
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
                    name: "ie_dma_end",
                    description: Some(
                        "enable interrupt for SPI DMA completion.",
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
                    name: "ie_fifo_ov",
                    description: Some(
                        "enable interrupt for SPI FIFO overflow.",
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
                    name: "ie_fst_byte",
                    description: Some(
                        "enable interrupt for SPI slave mode first byte received.",
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
            ],
        },
        FieldSet {
            name: "RunFlag",
            extends: None,
            description: Some(
                "SPI_FULL work flag.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slv_cmd_act",
                    description: Some(
                        "SPI slave command flag.",
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
                    name: "fifo_ready",
                    description: Some(
                        "SPI FIFO ready status.",
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
                    name: "slv_cs_load",
                    description: Some(
                        "SPI slave chip-select loading status.",
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
                    name: "slv_select",
                    description: Some(
                        "SPI slave selection status.",
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
            ],
        },
        FieldSet {
            name: "SlavePre",
            extends: None,
            description: Some(
                "SPI_FULL slave preset value.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slave_pre",
                    description: Some(
                        "slave preset value.",
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
            name: "TotalCnt",
            extends: None,
            description: Some(
                "SPI_FULL total byte count, only low 12 bit.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "total_cnt",
                    description: Some(
                        "SPI total byte count, only low 12 bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                