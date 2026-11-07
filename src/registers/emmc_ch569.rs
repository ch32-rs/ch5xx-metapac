
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Emmc",
            extends: None,
            description: Some(
                "EMMC register.",
            ),
            items: &[
                BlockItem {
                    name: "argument",
                    description: Some(
                        "SD 32bits command argument register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Argument",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_set",
                    description: Some(
                        "SD 16bits cmd setting register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "CmdSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "response0",
                    description: Some(
                        "SD 128bits response register, [31:0] 32bits.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Response0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "response1",
                    description: Some(
                        "SD 128bits response register, [63:32] 32bits.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Response1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "response2",
                    description: Some(
                        "SD 128bits response register, [95:64] 32bits.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Response2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "response3",
                    description: Some(
                        "SD 128bits response register, [127:96] 32bits.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Response3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "write_cont",
                    description: Some(
                        "Multiplexing register of the EMMC_RESPONSE3,[127:96] 32bits.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "WriteCont",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "control",
                    description: Some(
                        "SD 8bits control register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Control",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timeout",
                    description: Some(
                        "SD 8bits data timeout value.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Timeout",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "status",
                    description: Some(
                        "SD status.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Status",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_fg",
                    description: Some(
                        "SD 16bits interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "IntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "SD 16bits interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "IntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_beg1",
                    description: Some(
                        "SD 16bits DMA start address register when to operate.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaBeg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "block_cfg",
                    description: Some(
                        "SD 32bits data counter, [15:0] number of blocks this time will tran/recv, [27:16] block sise(byte number) of every block in this time tran/recv.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BlockCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tran_mode",
                    description: Some(
                        "SD TRANSFER MODE register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TranMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clk_div",
                    description: Some(
                        "SD clock divider register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "ClkDiv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_beg2",
                    description: Some(
                        "SD 16bits DMA start address register when to operate.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaBeg2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Argument",
            extends: None,
            description: Some(
                "SD 32bits command argument register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "argument",
                    description: Some(
                        "32 bit command parameter register.",
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
            name: "BlockCfg",
            extends: None,
            description: Some(
                "SD 32bits data counter, [15:0] number of blocks this time will tran/recv, [27:16] block sise(byte number) of every block in this time tran/recv.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bknum_mask",
                    description: Some(
                        "the number of blocks to be transferred.",
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
                Field {
                    name: "bksize_mask",
                    description: Some(
                        "single block transfer size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ClkDiv",
            extends: None,
            description: Some(
                "SD clock divider register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "div_mask",
                    description: Some(
                        "clk div.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clkoe",
                    description: Some(
                        "chip output sdclk oe.",
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
                    name: "clkmode",
                    description: Some(
                        "EMMC clock frequency mode selection bit.",
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
                Field {
                    name: "phaseinv",
                    description: Some(
                        "invert chip output sdclk phase.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CmdSet",
            extends: None,
            description: Some(
                "SD 16bits cmd setting register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "cmdidx_mask",
                    description: Some(
                        "the index number of the currently sent command.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rpty_mask",
                    description: Some(
                        "current respone type.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckcrc",
                    description: Some(
                        "check the response CRC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckidx",
                    description: Some(
                        "check the response command index.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Control",
            extends: None,
            description: Some(
                "SD 8bits control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "lw_mask",
                    description: Some(
                        "effctive data width for sending or receiving data.",
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
                    name: "all_clr",
                    description: Some(
                        "reset all the inner logic, default is valid.",
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
                    name: "dmaen",
                    description: Some(
                        "enable the dma.",
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
                    name: "rst_lgc",
                    description: Some(
                        "reset the data tran/recv logic.",
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
                    name: "negsmp",
                    description: Some(
                        "controller use nagedge sample cmd.",
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
            name: "DmaBeg1",
            extends: None,
            description: Some(
                "SD 16bits DMA start address register when to operate.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaad1_mask",
                    description: Some(
                        "start address of read-write data buffer,the lower 4 bits are fixed to 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaBeg2",
            extends: None,
            description: Some(
                "SD 16bits DMA start address register when to operate.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaad2_mask",
                    description: Some(
                        "block DMA start address register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntEn",
            extends: None,
            description: Some(
                "SD 16bits interrupt enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ie_re_tmout",
                    description: Some(
                        "command response timeout interrupt enable.",
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
                    name: "ie_recrc_wr",
                    description: Some(
                        "response CRC check error interrupt enable.",
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
                    name: "ie_reidx_er",
                    description: Some(
                        "response index check error interrupt enable.",
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
                    name: "ie_cmddone",
                    description: Some(
                        "command completion interrupt enable.",
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
                    name: "ie_dattmo",
                    description: Some(
                        "data timeout interrupt enable.",
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
                    name: "ie_tranerr",
                    description: Some(
                        "blocks transfer CRC error interrupt enable.",
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
                    name: "ie_trandone",
                    description: Some(
                        "all blocks transfer complete interrupt enable.",
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
                    name: "ie_bkgap",
                    description: Some(
                        "single block transmission completion interrupt enable.",
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
                    name: "ie_fifo_ov",
                    description: Some(
                        "FIFO overflow interrupt enable.",
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
                    name: "ie_sdioint",
                    description: Some(
                        "SDIO card interrupt enable.",
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
        FieldSet {
            name: "IntFg",
            extends: None,
            description: Some(
                "SD 16bits interrupt flag register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "if_re_tmout",
                    description: Some(
                        "indicate when expect the response, timeout.",
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
                    name: "if_recrc_wr",
                    description: Some(
                        "indicate CRC error of the response.",
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
                    name: "if_reidx_er",
                    description: Some(
                        "indicate INDEX error of the response.",
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
                    name: "if_cmddone",
                    description: Some(
                        "when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response.",
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
                    name: "if_dattmo",
                    description: Some(
                        "data line busy timeout.",
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
                    name: "if_tranerr",
                    description: Some(
                        "last block have encountered a CRC error.",
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
                    name: "if_trandone",
                    description: Some(
                        "all the blocks have been tran/recv successfully.",
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
                    name: "if_bkgap",
                    description: Some(
                        "every block gap interrupt when multiple read or write, allow drive change the DMA address at this moment.",
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
                    name: "if_fifo_ov",
                    description: Some(
                        "fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow.",
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
                    name: "if_sdioint",
                    description: Some(
                        "interrupt from SDIO card inside.",
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
        FieldSet {
            name: "Response0",
            extends: None,
            description: Some(
                "SD 128bits response register, [31:0] 32bits.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "response0",
                    description: Some(
                        "response parameter register.",
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
            name: "Response1",
            extends: None,
            description: Some(
                "SD 128bits response register, [63:32] 32bits.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "response1",
                    description: Some(
                        "response parameter register.",
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
            name: "Response2",
            extends: None,
            description: Some(
                "SD 128bits response register, [95:64] 32bits.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "response2",
                    description: Some(
                        "response parameter register.",
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
            name: "Response3",
            extends: None,
            description: Some(
                "SD 128bits response register, [127:96] 32bits.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "response3",
                    description: Some(
                        "response parameter register.",
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
                "SD status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mask_block_num",
                    description: Some(
                        "the number of blocks successfully transmitted in the current multi-block transmission.",
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
                Field {
                    name: "cmdsta",
                    description: Some(
                        "indicate cmd line is high level now.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dat0sta",
                    description: Some(
                        "indicate dat[0] line is high level now.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timeout",
            extends: None,
            description: Some(
                "SD 8bits data timeout value.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tocnt_mask",
                    description: Some(
                        "response /data timeout configuration.",
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
            ],
        },
        FieldSet {
            name: "TranMode",
            extends: None,
            description: Some(
                "SD TRANSFER MODE register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_dir",
                    description: Some(
                        "set DMA direction is controller to emmc card.",
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
                    name: "gap_stop",
                    description: Some(
                        "clock stop mode after block completion.",
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
                    name: "mode_boot",
                    description: Some(
                        "enable emmc boot mode.",
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
                    name: "autogapstop",
                    description: Some(
                        "enable auto set bTM_GAP_STOP when tran start.",
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
                    name: "fifo_rdy",
                    description: Some(
                        "FIFO ready select signal when writing EMMC.",
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
                Field {
                    name: "dmatn_cnt",
                    description: Some(
                        "in double buffer mode,set the block count value of buffer switch.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "duledma_en",
                    description: Some(
                        "enable double buffer dma.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "WriteCont",
            extends: None,
            description: Some(
                "Multiplexing register of the EMMC_RESPONSE3,[127:96] 32bits.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "write_cont",
                    description: Some(
                        "response parameter register.",
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
    ],
    enums: &[],
};
                