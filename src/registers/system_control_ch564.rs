
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
                    name: "wdog_clear",
                    description: Some(
                        "watch-dog clear register.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 8,
                            fieldset: Some(
                                "WdogClear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glob_mem_cfg",
                    description: Some(
                        "Memory configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "GlobMemCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glob_lock_port",
                    description: Some(
                        "Global port lock configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "GlobLockPort",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glob_rst_cfg",
                    description: Some(
                        "Globally reset the configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "GlobRstCfg",
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
            name: "GlobLockPort",
            extends: None,
            description: Some(
                "Global port lock configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "glob_lock_pa",
                    description: Some(
                        "PA port lock configuration.",
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
                    name: "glob_lock_pb",
                    description: Some(
                        "PB port lock configuration.",
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
                    name: "glob_lock_pd",
                    description: Some(
                        "PD port lock configuration.",
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
            name: "GlobMemCfg",
            extends: None,
            description: Some(
                "Memory configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "glob_mem_cfg",
                    description: Some(
                        "Memory configuration.",
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
                    name: "glob_cfg_flag",
                    description: Some(
                        "Memory configuration flag.",
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
            name: "GlobRstCfg",
            extends: None,
            description: Some(
                "Globally reset the configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "glob_wdog_en",
                    description: Some(
                        "enable watch-dog reset.",
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
                    name: "safe_access_sig",
                    description: Some(
                        "safe accessing sign register.",
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
            name: "WdogClear",
            extends: None,
            description: Some(
                "watch-dog clear register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wdog_clear",
                    description: Some(
                        "watch-dog clear.",
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
                