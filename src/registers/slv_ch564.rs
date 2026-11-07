use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Slv",
            extends: None,
            description: Some(
                "SLV register.",
            ),
            items: &[
                BlockItem {
                    name: "config",
                    description: Some(
                        "SLV configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Config",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dout",
                    description: Some(
                        "SLV data output register.",
                    ),
                    array: None,
                    byte_offset: 0x16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Dout",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "status",
                    description: Some(
                        "SLV status output register.",
                    ),
                    array: None,
                    byte_offset: 0x17,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Status",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_flag",
                    description: Some(
                        "SLV interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x42,
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
                    name: "int_din",
                    description: Some(
                        "SLV data input register.",
                    ),
                    array: None,
                    byte_offset: 0x43,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntDin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_en",
                    description: Some(
                        "SLV DMA enable register.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DmaEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_mode_ctrl",
                    description: Some(
                        "SLV DMA mode control register.",
                    ),
                    array: None,
                    byte_offset: 0x181,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DmaModeCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_mode_en",
                    description: Some(
                        "SLV DMA mode enable register.",
                    ),
                    array: None,
                    byte_offset: 0x182,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DmaModeEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_int_flag",
                    description: Some(
                        "SLV DMA interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DmaIntFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wr_dma_start_addr",
                    description: Some(
                        "DMA receive mode start address register.",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WrDmaStartAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wr_dma_end_addr",
                    description: Some(
                        "DMA receive mode end address register.",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WrDmaEndAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rd_dma_start_addr",
                    description: Some(
                        "DMA send mode start address register.",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RdDmaStartAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rd_dma_end_addr",
                    description: Some(
                        "DMA send mode end address register.",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RdDmaEndAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_end_now",
                    description: Some(
                        "DMA current address register.",
                    ),
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaEndNow",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_cmd0",
                    description: Some(
                        "DMA mode command word0 register.",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DmaCmd0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_cmd1",
                    description: Some(
                        "DMA mode command word1 register.",
                    ),
                    array: None,
                    byte_offset: 0x1a1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DmaCmd1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reset_cmd",
                    description: Some(
                        "DMA reset the command word register.",
                    ),
                    array: None,
                    byte_offset: 0x1a4,
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
                        "SLV busy flag register.",
                    ),
                    array: None,
                    byte_offset: 0x1a5,
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
                    name: "other_data",
                    description: Some(
                        "Non-DMA writes to the data register.",
                    ),
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "OtherData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_dec_len",
                    description: Some(
                        "DMA parses length register.",
                    ),
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "DmaDecLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_dec_offset",
                    description: Some(
                        "DMA resolves the offset address register.",
                    ),
                    array: None,
                    byte_offset: 0x1b6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "DmaDecOffset",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Busy",
            extends: None,
            description: Some(
                "SLV busy flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "busy",
                    description: Some(
                        "SLV busy flag.",
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
            name: "Config",
            extends: None,
            description: Some(
                "SLV configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "SLV enable bit.",
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
                    name: "ie_cmd",
                    description: Some(
                        "The command writes the interrupt enable bit.",
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
                    name: "ie_wr",
                    description: Some(
                        "Data writes the interrupt enable bit.",
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
                    name: "ie_rd",
                    description: Some(
                        "Data read the interrupt enable bit.",
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
            name: "DmaCmd0",
            extends: None,
            description: Some(
                "DMA mode command word0 register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_cmd0_slv",
                    description: Some(
                        "command word required to start DMA mode.",
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
            name: "DmaCmd1",
            extends: None,
            description: Some(
                "DMA mode command word1 register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_cmd1_slv",
                    description: Some(
                        "command word required to start DMA mode.",
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
            name: "DmaDecLen",
            extends: None,
            description: Some(
                "DMA parses length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dma_dec_len",
                    description: Some(
                        "parse the resulting DMA length.",
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
            name: "DmaDecOffset",
            extends: None,
            description: Some(
                "DMA resolves the offset address register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dma_dec_offset",
                    description: Some(
                        "Resolved DMA address offset.",
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
            name: "DmaEn",
            extends: None,
            description: Some(
                "SLV DMA enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_en_slv",
                    description: Some(
                        "SLV DMA mode enable control bit.",
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
                    name: "dma_resp_ie_slv",
                    description: Some(
                        "SLV DMA error transmission interrupts enable control bit.",
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
                    name: "dma_end_ie_slv",
                    description: Some(
                        "SLV DMA error transmission end enable control bit.",
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
                    name: "dma_loop_en_slv",
                    description: Some(
                        "SLV DMA cyclic address mode control bits.",
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
                    name: "dma_fifo_ov_ie_slv",
                    description: Some(
                        "The FIFO full state interrupts the enable bit.",
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
                    name: "dma_other_ie_slv",
                    description: Some(
                        "when in compatibility mode, Transmission of non-DMA data. state interrupts the enable bit.",
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
                    name: "dma_wr_8bit",
                    description: Some(
                        "selection of transfer data length for DMA write operations.",
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
            name: "DmaEndNow",
            extends: None,
            description: Some(
                "DMA current address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_now_addr_slv",
                    description: Some(
                        "DMA current address.",
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
            name: "DmaIntFlag",
            extends: None,
            description: Some(
                "SLV DMA interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wr_dma_end_if",
                    description: Some(
                        "DMA receive mode transmission end flag bit.",
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
                    name: "wr_dma_resp_if",
                    description: Some(
                        "DMA receive mode transmission error flag bit.",
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
                    name: "rd_dma_end_if",
                    description: Some(
                        "DMA occurrence mode transmission end flag bit.",
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
                    name: "rd_dma_resp_if",
                    description: Some(
                        "DMA occurrence mode transmission error flag bit.",
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
                    name: "wr_dma_ov_if",
                    description: Some(
                        "input FIFO full flag.",
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
                    name: "wr_other_if",
                    description: Some(
                        "data are written to the flag bit.",
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
                    name: "data_in_fifo_empty",
                    description: Some(
                        "Whether DATA_IN_FIFO is empty.",
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
            name: "DmaModeCtrl",
            extends: None,
            description: Some(
                "SLV DMA mode control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "cmd0_wr_mode",
                    description: Some(
                        "DMA transfer command word 0 read and write type.",
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
                    name: "cmd1_wr_mode",
                    description: Some(
                        "DMA transfer command word 1 read and write type.",
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
            name: "DmaModeEn",
            extends: None,
            description: Some(
                "SLV DMA mode enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "cmd0_en",
                    description: Some(
                        "DMA transfer command word 0 control bit.",
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
                    name: "cmd1_en",
                    description: Some(
                        "DMA transfer command word 1 control bit.",
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
                    name: "dma_cmd_mode1",
                    description: Some(
                        "DMA transfer command word mode1.",
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
            name: "Dout",
            extends: None,
            description: Some(
                "SLV data output register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dout",
                    description: Some(
                        "SLV data output.",
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
            name: "IntDin",
            extends: None,
            description: Some(
                "SLV data input register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "int_slv_din",
                    description: Some(
                        "SLV data input.",
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
                "SLV interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "if_slv_cmd1",
                    description: Some(
                        "The state of the original SLV address line 1.",
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
                    name: "if_slv_cmd1_sync",
                    description: Some(
                        "The status of the SLV address line 1 after synchronization.",
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
                    name: "if_slv_cmd0",
                    description: Some(
                        "The original SLV command input flag bit.",
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
                    name: "if_slv_cmd",
                    description: Some(
                        "The synchronized SLV command input flag bits.",
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
                    name: "if_slv_wr",
                    description: Some(
                        "SLV data input interrupt flag bit.",
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
                    name: "if_slv_rd",
                    description: Some(
                        "SLV data output interrupt flag bit.",
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
            name: "OtherData",
            extends: None,
            description: Some(
                "Non-DMA writes to the data register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "other_data",
                    description: Some(
                        "Non-DMA writes to the data.",
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
            name: "RdDmaEndAddr",
            extends: None,
            description: Some(
                "DMA send mode end address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_dma_start_addr_slv",
                    description: Some(
                        "DMA send mode end address.",
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
            name: "RdDmaStartAddr",
            extends: None,
            description: Some(
                "DMA send mode start address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_dma_start_addr_slv",
                    description: Some(
                        "DMA send mode start address.",
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
            name: "ResetCmd",
            extends: None,
            description: Some(
                "DMA reset the command word register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "reset_cmd",
                    description: Some(
                        "A global reset is generated upon receipt of this command.",
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
            name: "Status",
            extends: None,
            description: Some(
                "SLV status output register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "status",
                    description: Some(
                        "SLV status output.",
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
            name: "WrDmaEndAddr",
            extends: None,
            description: Some(
                "DMA receive mode end address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wr_dma_start_addr_slv",
                    description: Some(
                        "DMA receive mode end address.",
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
            name: "WrDmaStartAddr",
            extends: None,
            description: Some(
                "DMA receive mode start address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wr_dma_start_addr_slv",
                    description: Some(
                        "DMA receive mode start address.",
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
    ],
    enums: &[],
};
