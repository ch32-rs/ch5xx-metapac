
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
                                "CtrlModFull",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl_dma",
                    description: Some(
                        "SPI_FULL DMA control.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "CtrlDmaFull",
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
                                "InterEnFull",
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
                        "SPI_FULL work flag.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "RunFlagFull",
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
                                "IntFlagFull",
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
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "FifoCount",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reset_cmd",
                    description: Some(
                        "SPI_FULL reset command word register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ResetCmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "busy",
                    description: Some(
                        "SPI_FULL busy flag register.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Busy",
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
                        "SPI_NODMA mode control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "CtrlModNodma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl_dma",
                    description: Some(
                        "SPI_NODMA DMA control.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "CtrlDmaNodma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "inter_en",
                    description: Some(
                        "SPI_NODMA interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "InterEnNodma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clock_div",
                    description: Some(
                        "SPI_NODMA master clock divisor.",
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
                        "SPI_NODMA data buffer.",
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
                        "SPI_NODMA work flag.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "RunFlagNodma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_flag",
                    description: Some(
                        "SPI_NODMA interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntFlagNodma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifo_count",
                    description: Some(
                        "SPI_NODMA FIFO count status.",
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
                        "SPI_NODMA total byte count, only low 12 bit.",
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
                        "SPI_NODMA FIFO register.",
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
                        "SPI_NODMA FIFO count status.",
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
            name: "Busy",
            extends: None,
            description: Some(
                "SPI_FULL busy flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "busy",
                    description: Some(
                        "busy flag bit.",
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
            name: "CtrlDmaFull",
            extends: None,
            description: Some(
                "SPI_FULL DMA control.",
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
                    name: "dma_burst",
                    description: Some(
                        "DMA block operates on the enable bit.",
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
                    name: "hs_host",
                    description: Some(
                        "High speed host receive mode control bit.",
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
            name: "CtrlDmaNodma",
            extends: None,
            description: Some(
                "SPI_NODMA DMA control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "hs_host",
                    description: Some(
                        "High speed host receive mode control bit.",
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
            name: "CtrlModFull",
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
                    name: "mst_sck_mod",
                    description: Some(
                        "SPI master clock mode.",
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
                        "SPI slave command mode.",
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
            name: "CtrlModNodma",
            extends: None,
            description: Some(
                "SPI_NODMA mode control.",
            ),
            bit_size: 8,
            fields: &[
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
                    name: "slv_cmd_mod",
                    description: Some(
                        "SPI slave command mode.",
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
                        "SPI 2 wire or 3 wire SPI mode configuration bits.",
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
            name: "IntFlagFull",
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
                    name: "if_dma_err",
                    description: Some(
                        "interrupt flag for SPI DMA transfer error.",
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
            name: "IntFlagNodma",
            extends: None,
            description: Some(
                "SPI_NODMA interrupt flag.",
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
            ],
        },
        FieldSet {
            name: "InterEnFull",
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
                        "enable interrupt for SPI single byte exchanged.",
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
                    name: "ie_dma_err",
                    description: Some(
                        "enable interrupt for DMA transfer error.",
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
            name: "InterEnNodma",
            extends: None,
            description: Some(
                "SPI_NODMA interrupt enable.",
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
                        "enable interrupt for SPI single byte exchanged.",
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
            ],
        },
        FieldSet {
            name: "ResetCmd",
            extends: None,
            description: Some(
                "SPI_FULL reset command word register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "reset_cmd",
                    description: Some(
                        "A global reset is generated when this command is received.",
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
            name: "RunFlagFull",
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
            name: "RunFlagNodma",
            extends: None,
            description: Some(
                "SPI_NODMA work flag.",
            ),
            bit_size: 8,
            fields: &[
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
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                