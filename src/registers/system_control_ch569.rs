
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "SystemControl",
            extends: None,
            description: Some(
                "System_control register.",
            ),
            items: &[
                BlockItem {
                    name: "safe_access_sig",
                    description: Some(
                        "safe accessing sign register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SafeAccessSig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chip_id",
                    description: Some(
                        "chip ID register.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "ChipId",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "safe_access_id",
                    description: Some(
                        "safe accessing ID register.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "SafeAccessId",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdog_count",
                    description: Some(
                        "watch-dog count register.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "WdogCount",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glob_rom_cfg",
                    description: Some(
                        "flash ROM configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "GlobRomCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rst_boot_stat",
                    description: Some(
                        "reset status and boot/debug status.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "RstBootStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rst_wdog_ctrl",
                    description: Some(
                        "reset and watch-dog control.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "RstWdogCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glob_reset_keep",
                    description: Some(
                        "value keeper during global reset.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "GlobResetKeep",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ChipId",
            extends: None,
            description: Some(
                "chip ID register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "chip_id",
                    description: Some(
                        "chip ID.",
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
            name: "GlobResetKeep",
            extends: None,
            description: Some(
                "value keeper during global reset.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "glob_reset_keep",
                    description: Some(
                        "value keeper during global reset.",
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
            name: "GlobRomCfg",
            extends: None,
            description: Some(
                "flash ROM configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rom_ext_re",
                    description: Some(
                        "enable flash ROM being read by external programmer.",
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
                    name: "code_ram_we",
                    description: Some(
                        "enable code RAM being write.",
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
                    name: "rom_data_we",
                    description: Some(
                        "enable flash ROM data area being erase/write.",
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
                    name: "rom_code_we",
                    description: Some(
                        "enable flash ROM code and data area being erase or write.",
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
                    name: "rom_code_ofs",
                    description: Some(
                        "Config the start offset address of user code in Flash.",
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
            name: "RstBootStat",
            extends: None,
            description: Some(
                "reset status and boot/debug status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "reset_flag",
                    description: Some(
                        "recent reset flag.",
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
                    name: "cfg_reset_en",
                    description: Some(
                        "manual reset input enable status.",
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
                    name: "cfg_boot_en",
                    description: Some(
                        "boot-loader enable status.",
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
                    name: "cfg_debug_en",
                    description: Some(
                        "debug enable status.",
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
                    name: "boot_loader",
                    description: Some(
                        "indicate boot loader status.",
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
            name: "RstWdogCtrl",
            extends: None,
            description: Some(
                "reset and watch-dog control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "software_reset",
                    description: Some(
                        "global software reset.",
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
                    name: "wdog_rst_en",
                    description: Some(
                        "enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow.",
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
                    name: "wdog_int_en",
                    description: Some(
                        "watch-dog interrupt enable or INT_ID_WDOG interrupt source selection: 0=software interrupt.",
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
                    name: "wdog_int_flag",
                    description: Some(
                        "watch-dog timer overflow interrupt flag.",
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
            name: "SafeAccessId",
            extends: None,
            description: Some(
                "safe accessing ID register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "safe_access_id",
                    description: Some(
                        "safe accessing ID.",
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
            name: "SafeAccessSig",
            extends: None,
            description: Some(
                "safe accessing sign register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "safe_acc_mode",
                    description: Some(
                        "current safe accessing mode.",
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
                    name: "safe_acc_timer",
                    description: Some(
                        "safe accessing timer bit mask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "WdogCount",
            extends: None,
            description: Some(
                "watch-dog count register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wdog_count",
                    description: Some(
                        "watch-dog count.",
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
    ],
    enums: &[],
};
                