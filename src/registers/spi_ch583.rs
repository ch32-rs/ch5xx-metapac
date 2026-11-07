
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
                        "RW, SPI_FULL mode control.",
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
                        "RW, SPI_FULL configuration control.",
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
                        "RW, SPI_FULL interrupt enable.",
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
                        "RW, SPI_FULL master clock divisor.",
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
                        "RW, SPI_FULL slave preset value.",
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
                        "RW, SPI_FULL data buffer.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "RO, SPI_FULL work flag.",
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
                        "RW1, SPI_FULL interrupt flag.",
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
                        "RO, SPI_FULL FIFO count status.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "RW, SPI_FULL total byte count, only low 12 bit.",
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
                        "RO/WO, SPI_FULL FIFO register.",
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
                        "RO, SPI_FULL FIFO count status.",
                    ),
                    array: None,
                    byte_offset: 0x13,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "RW, SPI_FULL DMA current address.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "DmaNow",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_beg",
                    description: Some(
                        "RW, SPI_FULL DMA begin address.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "DmaBeg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_end",
                    description: Some(
                        "RW, SPI_FULL DMA end address.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "DmaEnd",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "SpiNodma",
            extends: None,
            description: Some(
                "SPI_NODMA register.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl_mod",
                    description: Some(
                        "RW, SPI_NODMA mode control.",
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
                        "RW, SPI_NODMA configuration control.",
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
                        "RW, SPI_NODMA interrupt enable.",
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
                        "RW, SPI_NODMA master clock divisor.",
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
                    name: "buffer",
                    description: Some(
                        "RW, SPI_NODMA data buffer.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "RO, SPI_NODMA work flag.",
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
                        "RW1, SPI_NODMA interrupt flag.",
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
                        "RO, SPI_NODMA FIFO count status.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "RW, SPI_NODMA total byte count, only low 12 bit.",
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
                        "RO/WO, SPI_NODMA FIFO register.",
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
                        "RO, SPI_NODMA FIFO count status.",
                    ),
                    array: None,
                    byte_offset: 0x13,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "FifoCount1",
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
                "RW, SPI_FULL data buffer.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "buffer",
                    description: Some(
                        "RW, SPI_FULL data buffer.",
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
                "RW, SPI_FULL master clock divisor.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clock_div",
                    description: Some(
                        "RW, SPI_FULL master clock divisor.",
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
                "RW, SPI_FULL configuration control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_enable",
                    description: Some(
                        "RW, SPI_FULL DMA enable.",
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
                        "RW, SPI_FULL DMA address loop enable.",
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
                        "RW, enable buffer/FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag.",
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
                        "RW, SPI bit data order: 0=MSB first, 1=LSB first.",
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
                    name: "mst_dly_en",
                    description: Some(
                        "RW, SPI master input delay enable.",
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
            ],
        },
        FieldSet {
            name: "CtrlMod",
            extends: None,
            description: Some(
                "RW, SPI_FULL mode control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mode_slave",
                    description: Some(
                        "RW, SPI_FULL slave mode: 0=master or host, 1=slave or device.",
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
                        "RW, force clear SPI FIFO and count.",
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
                        "RW, SPI_FULL enable 2 wire mode for slave: 0=3wire(SCK0,MOSI,MISO), 1=2wire(SCK0,MISO=MXSX).",
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
                    name: "mst_sck_mod",
                    description: Some(
                        "RW, SPI master clock mode: 0=mode 0, 1=mode 3.",
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
                    name: "slv_cmd_mod",
                    description: Some(
                        "RW, SPI_FULL slave command mode: 0=byte stream, 1=first byte command.",
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
                        "RW, SPI FIFO direction: 0=out(write @master mode), 1=in(read @master mode).",
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
                        "RW, SPI SCK output enable.",
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
                        "RW, SPI MOSI output enable.",
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
                        "RW, SPI MISO output enable.",
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
                "RW, SPI_FULL DMA begin address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dma_beg",
                    description: Some(
                        "RW, SPI_FULL DMA begin address.",
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
        FieldSet {
            name: "DmaEnd",
            extends: None,
            description: Some(
                "RW, SPI_FULL DMA end address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dma_end",
                    description: Some(
                        "RW, SPI_FULL DMA end address.",
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
        FieldSet {
            name: "DmaNow",
            extends: None,
            description: Some(
                "RW, SPI_FULL DMA current address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dma_now",
                    description: Some(
                        "RW, SPI_FULL DMA current address.",
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
        FieldSet {
            name: "Fifo",
            extends: None,
            description: Some(
                "RO/WO, SPI_FULL FIFO register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "fifo",
                    description: Some(
                        "RO/WO, SPI_FULL FIFO register.",
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
                "RO, SPI_FULL FIFO count status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "fifo_count",
                    description: Some(
                        "RO, SPI_FULL FIFO count status.",
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
                "RO, SPI_FULL FIFO count status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "fifo_count1",
                    description: Some(
                        "RO, SPI_FULL FIFO count status.",
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
                "RW1, SPI_FULL interrupt flag.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "if_cnt_end",
                    description: Some(
                        "RW1, interrupt flag for SPI total byte count end.",
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
                        "RW1, interrupt flag for SPI byte exchanged.",
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
                        "RW1, interrupt flag for SPI FIFO half.",
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
                        "RW1, interrupt flag for SPI_FULL DMA completion.",
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
                        "RW1, interrupt flag for SPI_FULL FIFO overflow.",
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
                        "RO, current SPI free status.",
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
                        "RW1, interrupt flag for SPI_FULL slave mode first byte received.",
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
                "RW, SPI_FULL interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ie_cnt_end",
                    description: Some(
                        "RW, enable interrupt for SPI total byte count end.",
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
                        "RW, enable interrupt for SPI byte exchanged.",
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
                        "RW, enable interrupt for SPI FIFO half.",
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
                        "RW, enable interrupt for SPI_FULL DMA completion.",
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
                        "RW, enable interrupt for SPI_FULL FIFO overflow.",
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
                        "RW, enable interrupt for SPI_FULL slave mode first byte received.",
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
                "RO, SPI_FULL work flag.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slv_cmd_act",
                    description: Some(
                        "RO, SPI_FULL slave first byte or command flag.",
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
                        "RO, SPI FIFO ready status.",
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
                        "RO, SPI_FULL slave chip-select loading status.",
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
                        "RO, SPI_FULL slave selection status.",
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
                "RW, SPI_FULL slave preset value.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slave_pre",
                    description: Some(
                        "RW, SPI_FULL slave preset value.",
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
                "RW, SPI_FULL total byte count, only low 12 bit.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "total_cnt",
                    description: Some(
                        "RW, SPI_FULL total byte count, only low 12 bit.",
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
                